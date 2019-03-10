use crate::Result;

use byteordered::Endian;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control0_1 {
  field_1: u16,
  field_2: u16,
}

impl Control0_1 {
  pub(crate) fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    Ok(Control0_1 {
      field_1: header.endianness().read_u16(&mut reader)?,
      field_2: header.endianness().read_u16(&mut reader)?,
    })
  }

  pub(crate) fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1)?;
    header.endianness().write_u16(&mut writer, self.field_2)?;

    Ok(())
  }
}
