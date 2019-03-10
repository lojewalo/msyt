use crate::{
  Result,
  model::Content,
};
use byteordered::Endian;
use msbt::Header;
use serde_derive::{Deserialize, Serialize};
use std::io::Write;

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
      let u = header.endianness().read_u16(chunk)?;
      skip += 1;
      if last_was_marker {
        let body = &s[i + 2..];
        let (read, ctl) = match u {
          0x00 => self::zero::Control0::parse(header, body).map(|(r, c)| (r, Control::Zero(c)))?,
          0x01 => self::one::Control1::parse(header, body).map(|(r, c)| (r, Control::One(c)))?,
          0x02 => self::two::Control2::parse(header, body).map(|(r, c)| (r, Control::Two(c)))?,
          0x03 => self::three::Control3::parse(header, body).map(|(r, c)| (r, Control::Three(c)))?,
          0x04 => self::four::Control4::parse(header, body).map(|(r, c)| (r, Control::Four(c)))?,
          0x05 => self::five::Control5::parse(header, body).map(|(r, c)| (r, Control::Five(c)))?,
          0xc9 => self::two_hundred_one::Control201::parse(header, body).map(|(r, c)| (r, Control::TwoHundredOne(c)))?,
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
            .map(|x| header.endianness().read_u16(x).map_err(Into::into))
            .collect::<Result<_>>()?;
          let string = String::from_utf16(&bytes)?;
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
      .map(|x| header.endianness().read_u16(x).map_err(Into::into))
      .collect::<Result<_>>()?;
    let string = String::from_utf16(&bytes)?;
    parts.push(Content::Text(string));
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
    header.endianness().write_u16(&mut writer, 0x0e)?;
    match *self {
      Control::Zero(ref c) => {
        header.endianness().write_u16(&mut writer, 0)?;
        c.write(header, &mut writer)
      },
      Control::One(ref c) => {
        header.endianness().write_u16(&mut writer, 1)?;
        c.write(header, &mut writer)
      },
      Control::Two(ref c) => {
        header.endianness().write_u16(&mut writer, 2)?;
        c.write(header, &mut writer)
      },
      Control::Three(ref c) => {
        header.endianness().write_u16(&mut writer, 3)?;
        c.write(header, &mut writer)
      },
      Control::Four(ref c) => {
        header.endianness().write_u16(&mut writer, 4)?;
        c.write(header, &mut writer)
      },
      Control::Five(ref c) => {
        header.endianness().write_u16(&mut writer, 5)?;
        c.write(header, &mut writer)
      },
      Control::TwoHundredOne(ref c) => {
        header.endianness().write_u16(&mut writer, 201)?;
        c.write(header, &mut writer)
      },
    }
  }
}
