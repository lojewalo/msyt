use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control3 {
  pub field_1: u16,
  pub field_2: Vec<u8>,
}

impl Control3 {
  pub fn parse(endianness: Endianness, buf: &[u8]) -> Result<(usize, Control3)> {
    let mut c = Cursor::new(buf);
    let field_1 = endianness.read_u16(&mut c)?;
    let field_2_len = endianness.read_u16(&mut c)?;
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
}
