use crate::Result;

use byteordered::Endian;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control5 {
  pub field_1: u16,
  pub field_2: u16,
}

impl Control5 {
  pub fn parse(header: &Header, buf: &[u8]) -> Result<(usize, Control5)> {
    let mut c = Cursor::new(buf);
    let field_1 = header.endianness().read_u16(&mut c)?;
    let field_2 = header.endianness().read_u16(&mut c)?;

    Ok((
      c.position() as usize,
      Control5 {
        field_1,
        field_2,
      }
    ))
  }

  pub fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1)?;
    header.endianness().write_u16(&mut writer, self.field_2)?;

    Ok(())
  }
}
