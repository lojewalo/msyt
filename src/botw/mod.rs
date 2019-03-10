use crate::{
  Result,
  model::Content,
};
use byteordered::Endian;
use failure::ResultExt;
use msbt::Header;
use serde_derive::{Deserialize, Serialize};
use std::io::{Cursor, Write};

pub mod zero;
pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;
pub mod two_hundred_one;

pub fn parse_controls(header: &Header, s: &[u8]) -> Result<Vec<Content>> {
  let mut parts = Vec::new();
  let mut last_was_marker = false;
  let mut skip = 0;
  let mut text_index = None;

  for i in 0..s.len() {
    if skip > 0 {
      skip -= 1;
      continue;
    }
    if i + 1 < s.len() {
      let chunk = &s[i..=i + 1];
      let u = header.endianness().read_u16(chunk).with_context(|_| "could not control sequence marker")?;
      skip += 1;
      if last_was_marker {
        let body = &s[i + 2..];
        let (read, ctl) = match u {
          0x00 => self::zero::Control0::parse(header, body).map(|(r, c)| (r, Control::Zero(c)))
            .with_context(|_| "could not parse control sequence 0")?,
          0x01 => self::one::Control1::parse(header, body).map(|(r, c)| (r, Control::One(c)))
            .with_context(|_| "could not parse control sequence 1")?,
          0x02 => self::two::Control2::parse(header, body).map(|(r, c)| (r, Control::Two(c)))
            .with_context(|_| "could not parse control sequence 2")?,
          0x03 => self::three::Control3::parse(header, body).map(|(r, c)| (r, Control::Three(c)))
            .with_context(|_| "could not parse control sequence 3")?,
          0x04 => self::four::Control4::parse(header, body).map(|(r, c)| (r, Control::Four(c)))
            .with_context(|_| "could not parse control sequence 4")?,
          0x05 => self::five::Control5::parse(header, body).map(|(r, c)| (r, Control::Five(c)))
            .with_context(|_| "could not parse control sequence 5")?,
          0xc9 => self::two_hundred_one::Control201::parse(header, body).map(|(r, c)| (r, Control::TwoHundredOne(c)))
            .with_context(|_| "could not parse control sequence 201")?,
          x => failure::bail!("unknown control sequence: {}", x),
        };
        let part = Content::Control(ctl);
        skip = read + 1;
        parts.push(part);
      }
      if text_index.is_none() && !last_was_marker && u != 0x0e {
        text_index = Some(i);
      }
      if u == 0x0e {
        last_was_marker = true;
        if let Some(text_index) = text_index {
          let bytes: Vec<u16> = s[text_index..i]
            .chunks(2)
            .map(|x| header.endianness().read_u16(x)
              .with_context(|_| "could not read bytes")
              .map_err(Into::into))
            .collect::<Result<_>>()?;
          let string = String::from_utf16(&bytes).with_context(|_| "could not parse utf-16 string")?;
          parts.push(Content::Text(string));
        }
        text_index = None;
      } else {
        last_was_marker = false;
      }
    }

  }

  if let Some(text_index) = text_index {
    let bytes: Vec<u16> = s[text_index..]
      .chunks(2)
      .map(|x| header.endianness().read_u16(x)
        .with_context(|_| "could not read bytes")
        .map_err(Into::into))
      .collect::<Result<_>>()?;
    let from = if bytes[bytes.len() - 1] == 0 {
      &bytes[..bytes.len() - 1]
    } else {
      &bytes
    };
    let string = String::from_utf16(&from).with_context(|_| "could not parse utf-16 string")?;
    if !string.is_empty() {
      parts.push(Content::Text(string));
    }
  }

  Ok(parts)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Control {
  Zero(self::zero::Control0),
  One(self::one::Control1),
  Two(self::two::Control2),
  Three(self::three::Control3),
  Four(self::four::Control4),
  Five(self::five::Control5),
  TwoHundredOne(self::two_hundred_one::Control201),
}

impl Control {
  pub fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, 0x0e).with_context(|_| "could not write control marker")?;
    let control = match *self {
      Control::Zero(ref c) => c as &MainControl,
      Control::One(ref c) => c as &MainControl,
      Control::Two(ref c) => c as &MainControl,
      Control::Three(ref c) => c as &MainControl,
      Control::Four(ref c) => c as &MainControl,
      Control::Five(ref c) => c as &MainControl,
      Control::TwoHundredOne(ref c) => c as &MainControl,
    };
    header.endianness().write_u16(&mut writer, control.marker())
      .with_context(|_| format!("could not write control marker for type {}", control.marker()))?;
    control.write(header, &mut writer)
      .with_context(|_| format!("could not write control type {}", control.marker()))
      .map_err(Into::into)
  }
}

pub(crate) trait MainControl {
  fn marker(&self) -> u16;

  fn parse(header: &Header, buf: &[u8]) -> Result<(usize, Self)>
    where Self: Sized;

  fn write(&self, header: &Header, writer: &mut Write) -> Result<()>;
}

pub(crate) trait SubControl {
  fn marker(&self) -> u16;

  fn parse(header: &Header, reader: &mut Cursor<&[u8]>) -> Result<Self>
    where Self: Sized;

  fn write(&self, header: &Header, writer: &mut Write) -> Result<()>;
}
