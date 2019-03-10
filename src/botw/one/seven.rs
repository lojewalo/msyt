use crate::{
  Result,
  botw::SubControl,
};

use byteordered::Endian;

use failure::ResultExt;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control1_7 {
  field_1: u16,
  field_2: [u8; 2],
}

impl SubControl for Control1_7 {
  fn marker(&self) -> u16 {
    7
  }

  fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let field_1 = header.endianness().read_u16(&mut reader).with_context(|_| "could not read field_1")?;

    let mut field_2 = [0; 2];
    reader.read_exact(&mut field_2).with_context(|_| "could not read field_2")?;

    Ok(Control1_7 {
      field_1,
      field_2,
    })
  }

  fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1).with_context(|_| "could not write field_1")?;
    writer.write_all(&self.field_2[..]).with_context(|_| "could not write field_2")?;

    Ok(())
  }
}
