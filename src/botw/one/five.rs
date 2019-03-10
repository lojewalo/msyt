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
pub struct Control1_5 {
  field_1: u16,
  field_2: u16,
  field_3: u16,
  field_4: u16,
  field_5: [u8; 2],
}

impl SubControl for Control1_5 {
  fn marker(&self) -> u16 {
    5
  }

  fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let mut field_5 = [0; 2];
    let field_1 = header.endianness().read_u16(&mut reader).with_context(|_| "could not read field_1")?;
    let field_2 = header.endianness().read_u16(&mut reader).with_context(|_| "could not read field_2")?;
    let field_3 = header.endianness().read_u16(&mut reader).with_context(|_| "could not read field_3")?;
    let field_4 = header.endianness().read_u16(&mut reader).with_context(|_| "could not read field_4")?;
    reader.read_exact(&mut field_5[..]).with_context(|_| "could not read field_5")?;

    Ok(Control1_5 {
      field_1,
      field_2,
      field_3,
      field_4,
      field_5,
    })
  }

  fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1).with_context(|_| "could not write field_1")?;
    header.endianness().write_u16(&mut writer, self.field_2).with_context(|_| "could not write field_2")?;
    header.endianness().write_u16(&mut writer, self.field_3).with_context(|_| "could not write field_3")?;
    header.endianness().write_u16(&mut writer, self.field_4).with_context(|_| "could not write field_4")?;
    writer.write_all(&self.field_5[..]).with_context(|_| "could not write field_5")?;

    Ok(())
  }
}
