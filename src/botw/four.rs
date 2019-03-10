use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::Cursor;

mod zero;
mod one;
mod two;
mod three;

use self::{
  zero::Control4_0,
  one::Control4_1,
  two::Control4_2,
  three::Control4_3,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Control4 {
  Type0(Control4_0),
  Type1(Control4_1),
  Type2(Control4_2),
  Type3(Control4_3),
}

impl Control4 {
  pub fn parse(endianness: Endianness, buf: &[u8]) -> Result<(usize, Self)> {
    let mut c = Cursor::new(buf);

    let kind = endianness.read_u16(&mut c)?;
    let control = match kind {
      0 => Control4::Type0(Control4_0::parse(endianness, &mut c)?),
      1 => Control4::Type1(Control4_1::parse(endianness, &mut c)?),
      2 => Control4::Type2(Control4_2::parse(endianness, &mut c)?),
      3 => Control4::Type3(Control4_3::parse(endianness, &mut c)?),
      x => failure::bail!("unknown control 4 type: {}", x),
    };

    Ok((
      c.position() as usize,
      control,
    ))
  }
}
