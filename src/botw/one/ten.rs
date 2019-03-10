use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control1_10 {
  field_1: u16,
  field_2: u16,
  field_3: [u8; 2],
}

impl Control1_10 {
  pub(crate) fn parse(endianness: Endianness, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let field_1 = endianness.read_u16(&mut reader)?;
    let field_2 = endianness.read_u16(&mut reader)?;

    let mut field_3 = [0; 2];
    reader.read_exact(&mut field_3[..])?;

    Ok(Control1_10 {
      field_1,
      field_2,
      field_3,
    })
  }
}
