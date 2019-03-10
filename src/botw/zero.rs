use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::Cursor;

#[allow(clippy::module_inception)]
mod zero;
mod one;
mod two;
mod three;
mod four;

use self::{
  zero::Control0_0,
  one::Control0_1,
  two::Control0_2,
  three::Control0_3,
  four::Control0_4,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Control0 {
  Type0(Control0_0),
  Type1(Control0_1),
  Type2(Control0_2),
  Type3(Control0_3),
  Type4(Control0_4),
}

impl Control0 {
  pub fn parse(endianness: Endianness, buf: &[u8]) -> Result<(usize, Self)> {
    let mut c = Cursor::new(buf);

    let kind = endianness.read_u16(&mut c)?;
    let control = match kind {
      0 => Control0::Type0(Control0_0::parse(endianness, &mut c)?),
      1 => Control0::Type1(Control0_1::parse(endianness, &mut c)?),
      2 => Control0::Type2(Control0_2::parse(endianness, &mut c)?),
      3 => Control0::Type3(Control0_3::parse(endianness, &mut c)?),
      4 => Control0::Type4(Control0_4::parse(endianness, &mut c)?),
      x => failure::bail!("unknown control 0 type: {}", x),
    };

    Ok((
      c.position() as usize,
      control,
    ))
  }
}
