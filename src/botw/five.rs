use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::Cursor;

#[derive(Debug, Deserialize, Serialize)]
pub struct Control5 {
  pub field_1: u16,
  pub field_2: u16,
}

impl Control5 {
  pub fn parse(endianness: Endianness, buf: &[u8]) -> Result<(usize, Control5)> {
    let mut c = Cursor::new(buf);
    let field_1 = endianness.read_u16(&mut c)?;
    let field_2 = endianness.read_u16(&mut c)?;

    Ok((
      c.position() as usize,
      Control5 {
        field_1,
        field_2,
      }
    ))
  }
}
