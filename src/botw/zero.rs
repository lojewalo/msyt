use crate::Result;

use byteordered::Endian;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

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
  pub fn parse(header: &Header, buf: &[u8]) -> Result<(usize, Self)> {
    let mut c = Cursor::new(buf);

    let kind = header.endianness().read_u16(&mut c)?;
    let control = match kind {
      0 => Control0::Type0(Control0_0::parse(header, &mut c)?),
      1 => Control0::Type1(Control0_1::parse(header, &mut c)?),
      2 => Control0::Type2(Control0_2::parse(header, &mut c)?),
      3 => Control0::Type3(Control0_3::parse(header, &mut c)?),
      4 => Control0::Type4(Control0_4::parse(header, &mut c)?),
      x => failure::bail!("unknown control 0 type: {}", x),
    };

    Ok((
      c.position() as usize,
      control,
    ))
  }

  pub fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    match *self {
      Control0::Type0(ref c) => {
        header.endianness().write_u16(&mut writer, 0)?;
        c.write(header, &mut writer)
      },
      Control0::Type1(ref c) => {
        header.endianness().write_u16(&mut writer, 1)?;
        c.write(header, &mut writer)
      },
      Control0::Type2(ref c) => {
        header.endianness().write_u16(&mut writer, 2)?;
        c.write(header, &mut writer)
      },
      Control0::Type3(ref c) => {
        header.endianness().write_u16(&mut writer, 3)?;
        c.write(header, &mut writer)
      },
      Control0::Type4(ref c) => {
        header.endianness().write_u16(&mut writer, 4)?;
        c.write(header, &mut writer)
      },
    }
  }
}
