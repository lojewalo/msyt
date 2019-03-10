use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::Cursor;

mod one_field;
mod variable;

use self::{
  one_field::Control2OneField,
  variable::Control2Variable,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Control2 {
  Type1(Control2Variable),
  Type2(Control2Variable),
  Type3(Control2OneField),
  Type4(Control2OneField),
  Type7(Control2OneField),
  Type8(Control2OneField),
  Type9(Control2Variable),
  Type10(Control2OneField),
  Type11(Control2Variable),
  Type13(Control2OneField),
  Type14(Control2Variable),
  Type15(Control2Variable),
  Type16(Control2Variable),
  Type17(Control2Variable),
  Type18(Control2Variable),
  Type19(Control2Variable),
}

impl Control2 {
  pub fn parse(endianness: Endianness, buf: &[u8]) -> Result<(usize, Self)> {
    let mut c = Cursor::new(buf);

    let kind = endianness.read_u16(&mut c)?;
    let control = match kind {
      1 => Control2::Type1(Control2Variable::parse(endianness, &mut c)?),
      2 => Control2::Type2(Control2Variable::parse(endianness, &mut c)?),
      3 => Control2::Type3(Control2OneField::parse(endianness, &mut c)?),
      4 => Control2::Type4(Control2OneField::parse(endianness, &mut c)?),
      7 => Control2::Type7(Control2OneField::parse(endianness, &mut c)?),
      8 => Control2::Type8(Control2OneField::parse(endianness, &mut c)?),
      9 => Control2::Type9(Control2Variable::parse(endianness, &mut c)?),
      10 => Control2::Type10(Control2OneField::parse(endianness, &mut c)?),
      11 => Control2::Type11(Control2Variable::parse(endianness, &mut c)?),
      13 => Control2::Type13(Control2OneField::parse(endianness, &mut c)?),
      14 => Control2::Type14(Control2Variable::parse(endianness, &mut c)?),
      15 => Control2::Type15(Control2Variable::parse(endianness, &mut c)?),
      16 => Control2::Type16(Control2Variable::parse(endianness, &mut c)?),
      17 => Control2::Type17(Control2Variable::parse(endianness, &mut c)?),
      18 => Control2::Type18(Control2Variable::parse(endianness, &mut c)?),
      19 => Control2::Type19(Control2Variable::parse(endianness, &mut c)?),
      x => failure::bail!("unknown control 2 type: {}", x),
    };

    Ok((
      c.position() as usize,
      control,
    ))
  }
}
