use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control1_7 {
  field_1: u16,
  field_2: [u8; 2],
}

impl Control1_7 {
  pub(crate) fn parse(endianness: Endianness, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let field_1 = endianness.read_u16(&mut reader)?;

    let mut field_2 = [0; 2];
    reader.read_exact(&mut field_2)?;

    Ok(Control1_7 {
      field_1,
      field_2,
    })
  }
}
