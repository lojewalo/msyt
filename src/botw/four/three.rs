use crate::{
  Result,
  botw::SubControl,
};

use byteordered::Endian;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control4_3 {
  field_1: u16,
}

impl SubControl for Control4_3 {
  fn marker(&self) -> u16 {
    3
  }

  fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    Ok(Control4_3 {
      field_1: header.endianness().read_u16(&mut reader)?,
    })
  }

  fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1)?;

    Ok(())
  }
}
