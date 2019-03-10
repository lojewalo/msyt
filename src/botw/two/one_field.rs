use crate::{
  Result,
  botw::SubControl,
};

use byteordered::Endian;

use failure::ResultExt;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control2OneField {
  field_1: u16,
}

impl SubControl for Control2OneField {
  fn marker(&self) -> u16 {
    unimplemented!("if you see this, this is a bug")
  }

  fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    Ok(Control2OneField {
      field_1: header.endianness().read_u16(&mut reader).with_context(|_| "could not read field_1")?,
    })
  }

  fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1).with_context(|_| "could not write field_1")?;

    Ok(())
  }
}
