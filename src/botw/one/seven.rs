use crate::Result;

use byteordered::Endian;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control1_7 {
  field_1: u16,
  field_2: [u8; 2],
}

impl Control1_7 {
  pub(crate) fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let field_1 = header.endianness().read_u16(&mut reader)?;

    let mut field_2 = [0; 2];
    reader.read_exact(&mut field_2)?;

    Ok(Control1_7 {
      field_1,
      field_2,
    })
  }

  pub(crate) fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1)?;
    writer.write_all(&self.field_2[..])?;

    Ok(())
  }
}
