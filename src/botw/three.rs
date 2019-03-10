use crate::Result;

use byteordered::Endian;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control3 {
  pub field_1: u16,
  pub field_2: Vec<u8>,
}

impl Control3 {
  pub fn parse(header: &Header, buf: &[u8]) -> Result<(usize, Control3)> {
    let mut c = Cursor::new(buf);
    let field_1 = header.endianness().read_u16(&mut c)?;
    let field_2_len = header.endianness().read_u16(&mut c)?;
    let mut field_2 = vec![0; field_2_len as usize];
    c.read_exact(&mut field_2)?;

    Ok((
      c.position() as usize,
      Control3 {
        field_1,
        field_2,
      }
    ))
  }

  pub fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1)?;
    header.endianness().write_u16(&mut writer, self.field_2.len() as u16)?;
    writer.write_all(&self.field_2)?;

    Ok(())
  }
}
