use crate::Result;

use byteordered::Endian;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

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
  pub fn parse(header: &Header, buf: &[u8]) -> Result<(usize, Self)> {
    let mut c = Cursor::new(buf);

    let kind = header.endianness().read_u16(&mut c)?;
    let control = match kind {
      1 => Control2::Type1(Control2Variable::parse(header, &mut c)?),
      2 => Control2::Type2(Control2Variable::parse(header, &mut c)?),
      3 => Control2::Type3(Control2OneField::parse(header, &mut c)?),
      4 => Control2::Type4(Control2OneField::parse(header, &mut c)?),
      7 => Control2::Type7(Control2OneField::parse(header, &mut c)?),
      8 => Control2::Type8(Control2OneField::parse(header, &mut c)?),
      9 => Control2::Type9(Control2Variable::parse(header, &mut c)?),
      10 => Control2::Type10(Control2OneField::parse(header, &mut c)?),
      11 => Control2::Type11(Control2Variable::parse(header, &mut c)?),
      13 => Control2::Type13(Control2OneField::parse(header, &mut c)?),
      14 => Control2::Type14(Control2Variable::parse(header, &mut c)?),
      15 => Control2::Type15(Control2Variable::parse(header, &mut c)?),
      16 => Control2::Type16(Control2Variable::parse(header, &mut c)?),
      17 => Control2::Type17(Control2Variable::parse(header, &mut c)?),
      18 => Control2::Type18(Control2Variable::parse(header, &mut c)?),
      19 => Control2::Type19(Control2Variable::parse(header, &mut c)?),
      x => failure::bail!("unknown control 2 type: {}", x),
    };

    Ok((
      c.position() as usize,
      control,
    ))
  }

  pub fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    match *self {
      Control2::Type1(ref c) => {
        header.endianness().write_u16(&mut writer, 1)?;
        c.write(header, &mut writer)
      },
      Control2::Type2(ref c) => {
        header.endianness().write_u16(&mut writer, 2)?;
        c.write(header, &mut writer)
      },
      Control2::Type3(ref c) => {
        header.endianness().write_u16(&mut writer, 3)?;
        c.write(header, &mut writer)
      },
      Control2::Type4(ref c) => {
        header.endianness().write_u16(&mut writer, 4)?;
        c.write(header, &mut writer)
      },
      Control2::Type7(ref c) => {
        header.endianness().write_u16(&mut writer, 7)?;
        c.write(header, &mut writer)
      },
      Control2::Type8(ref c) => {
        header.endianness().write_u16(&mut writer, 8)?;
        c.write(header, &mut writer)
      },
      Control2::Type9(ref c) => {
        header.endianness().write_u16(&mut writer, 9)?;
        c.write(header, &mut writer)
      },
      Control2::Type10(ref c) => {
        header.endianness().write_u16(&mut writer, 10)?;
        c.write(header, &mut writer)
      },
      Control2::Type11(ref c) => {
        header.endianness().write_u16(&mut writer, 11)?;
        c.write(header, &mut writer)
      },
      Control2::Type13(ref c) => {
        header.endianness().write_u16(&mut writer, 13)?;
        c.write(header, &mut writer)
      },
      Control2::Type14(ref c) => {
        header.endianness().write_u16(&mut writer, 14)?;
        c.write(header, &mut writer)
      },
      Control2::Type15(ref c) => {
        header.endianness().write_u16(&mut writer, 15)?;
        c.write(header, &mut writer)
      },
      Control2::Type16(ref c) => {
        header.endianness().write_u16(&mut writer, 16)?;
        c.write(header, &mut writer)
      },
      Control2::Type17(ref c) => {
        header.endianness().write_u16(&mut writer, 17)?;
        c.write(header, &mut writer)
      },
      Control2::Type18(ref c) => {
        header.endianness().write_u16(&mut writer, 18)?;
        c.write(header, &mut writer)
      },
      Control2::Type19(ref c) => {
        header.endianness().write_u16(&mut writer, 19)?;
        c.write(header, &mut writer)
      },
    }
  }
}
