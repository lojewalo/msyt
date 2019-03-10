use crate::{
  Result,
  model::Content,
};
use byteordered::{Endianness, Endian};
use serde_derive::{Deserialize, Serialize};

pub mod zero;
pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;
pub mod two_hundred_one;

pub fn parse_controls(endianness: Endianness, s: &[u8]) -> Result<Vec<Content>> {
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
      let u = endianness.read_u16(chunk)?;
      skip += 1;
      if last_was_marker {
        let body = &s[i + 2..];
        let (read, ctl) = match u {
          0x00 => self::zero::Control0::parse(endianness, body).map(|(r, c)| (r, Control::Zero(c)))?,
          0x01 => self::one::Control1::parse(endianness, body).map(|(r, c)| (r, Control::One(c)))?,
          0x02 => self::two::Control2::parse(endianness, body).map(|(r, c)| (r, Control::Two(c)))?,
          0x03 => self::three::Control3::parse(endianness, body).map(|(r, c)| (r, Control::Three(c)))?,
          0x04 => self::four::Control4::parse(endianness, body).map(|(r, c)| (r, Control::Four(c)))?,
          0x05 => self::five::Control5::parse(endianness, body).map(|(r, c)| (r, Control::Five(c)))?,
          0xc9 => self::two_hundred_one::Control201::parse(endianness, body).map(|(r, c)| (r, Control::TwoHundredOne(c)))?,
          x => (0, Control::Unknown(x)),
        };
        if let Control::Unknown(x) = ctl {
          println!("unknown control sequence: {}", x);
        } else {
          let part = Content::Control(ctl);
          skip = read + 1;
          parts.push(part);
        }
      }
      if text_index.is_none() && !last_was_marker && u != 0x0e {
        text_index = Some(i);
      }
      if u == 0x0e {
        last_was_marker = true;
        if let Some(text_index) = text_index {
          let bytes: Vec<u16> = s[text_index..i]
            .chunks(2)
            .map(|x| endianness.read_u16(x).map_err(Into::into))
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
      .map(|x| endianness.read_u16(x).map_err(Into::into))
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
  Unknown(u16),
}
