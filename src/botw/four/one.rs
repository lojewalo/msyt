use crate::Result;

use byteordered::Endian;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control4_1 {
  field_1: Vec<u8>,
}

impl Control4_1 {
  pub(crate) fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let field_1_len = header.endianness().read_u16(&mut reader)?;
    let mut field_1 = vec![0; field_1_len as usize];
    reader.read_exact(&mut field_1)?;

    Ok(Control4_1 {
      field_1,
    })
  }

  pub(crate) fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1.len() as u16)?;
    writer.write_all(&self.field_1)?;

    Ok(())
  }
}
