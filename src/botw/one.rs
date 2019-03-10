use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::Cursor;

mod zero;
#[allow(clippy::module_inception)]
mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod seven;
mod eight;
mod nine;
mod ten;

use self::{
  zero::Control1_0,
  one::Control1_1,
  two::Control1_2,
  three::Control1_3,
  four::Control1_4,
  five::Control1_5,
  six::Control1_6,
  seven::Control1_7,
  eight::Control1_8,
  nine::Control1_9,
  ten::Control1_10,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Control1 {
  Type0(Control1_0),
  Type1(Control1_1),
  Type2(Control1_2),
  Type3(Control1_3),
  Type4(Control1_4),
  Type5(Control1_5),
  Type6(Control1_6),
  Type7(Control1_7),
  Type8(Control1_8),
  Type9(Control1_9),
  Type10(Control1_10),
}

impl Control1 {
  pub fn parse(endianness: Endianness, buf: &[u8]) -> Result<(usize, Self)> {
    let mut c = Cursor::new(buf);

    let kind = endianness.read_u16(&mut c)?;
    let control = match kind {
      0 => Control1::Type0(Control1_0::parse(endianness, &mut c)?),
      1 => Control1::Type1(Control1_1::parse(endianness, &mut c)?),
      2 => Control1::Type2(Control1_2::parse(endianness, &mut c)?),
      3 => Control1::Type3(Control1_3::parse(endianness, &mut c)?),
      4 => Control1::Type4(Control1_4::parse(endianness, &mut c)?),
      5 => Control1::Type5(Control1_5::parse(endianness, &mut c)?),
      6 => Control1::Type6(Control1_6::parse(endianness, &mut c)?),
      7 => Control1::Type7(Control1_7::parse(endianness, &mut c)?),
      8 => Control1::Type8(Control1_8::parse(endianness, &mut c)?),
      9 => Control1::Type9(Control1_9::parse(endianness, &mut c)?),
      10 => Control1::Type10(Control1_10::parse(endianness, &mut c)?),
      x => failure::bail!("unknown control 1 type: {}", x),
    };

    Ok((
      c.position() as usize,
      control,
    ))
  }
}
