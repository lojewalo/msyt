use crate::Result;

use byteordered::Endian;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control1_4 {
  field_1: u16,
  field_2: u16,
  field_3: u16,
  field_4: [u8; 2],
}

impl Control1_4 {
  pub(crate) fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let mut field_4 = [0; 2];
    let field_1 = header.endianness().read_u16(&mut reader)?;
    let field_2 = header.endianness().read_u16(&mut reader)?;
    let field_3 = header.endianness().read_u16(&mut reader)?;
    reader.read_exact(&mut field_4[..])?;

    Ok(Control1_4 {
      field_1,
      field_2,
      field_3,
      field_4,
    })
  }

  pub(crate) fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1)?;
    header.endianness().write_u16(&mut writer, self.field_2)?;
    header.endianness().write_u16(&mut writer, self.field_3)?;
    writer.write_all(&self.field_4[..])?;

    Ok(())
  }
}
