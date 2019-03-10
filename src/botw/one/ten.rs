use crate::Result;

use byteordered::Endian;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control1_10 {
  field_1: u16,
  field_2: u16,
  field_3: [u8; 2],
}

impl Control1_10 {
  pub(crate) fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let field_1 = header.endianness().read_u16(&mut reader)?;
    let field_2 = header.endianness().read_u16(&mut reader)?;

    let mut field_3 = [0; 2];
    reader.read_exact(&mut field_3[..])?;

    Ok(Control1_10 {
      field_1,
      field_2,
      field_3,
    })
  }

  pub(crate) fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1)?;
    header.endianness().write_u16(&mut writer, self.field_2)?;
    writer.write_all(&self.field_3[..])?;

    Ok(())
  }
}
