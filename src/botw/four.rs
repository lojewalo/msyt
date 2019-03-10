use crate::Result;

use byteordered::Endian;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

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
  pub fn parse(header: &Header, buf: &[u8]) -> Result<(usize, Self)> {
    let mut c = Cursor::new(buf);

    let kind = header.endianness().read_u16(&mut c)?;
    let control = match kind {
      0 => Control4::Type0(Control4_0::parse(header, &mut c)?),
      1 => Control4::Type1(Control4_1::parse(header, &mut c)?),
      2 => Control4::Type2(Control4_2::parse(header, &mut c)?),
      3 => Control4::Type3(Control4_3::parse(header, &mut c)?),
      x => failure::bail!("unknown control 4 type: {}", x),
    };

    Ok((
      c.position() as usize,
      control,
    ))
  }

  pub fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    match *self {
      Control4::Type0(ref c) => {
        header.endianness().write_u16(&mut writer, 0)?;
        c.write(header, &mut writer)
      },
      Control4::Type1(ref c) => {
        header.endianness().write_u16(&mut writer, 1)?;
        c.write(header, &mut writer)
      },
      Control4::Type2(ref c) => {
        header.endianness().write_u16(&mut writer, 2)?;
        c.write(header, &mut writer)
      },
      Control4::Type3(ref c) => {
        header.endianness().write_u16(&mut writer, 3)?;
        c.write(header, &mut writer)
      },
    }
  }
}
