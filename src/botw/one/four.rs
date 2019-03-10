use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control1_4 {
  field_1: u16,
  field_2: u16,
  field_3: u16,
  field_4: [u8; 2],
}

impl Control1_4 {
  pub(crate) fn parse(endianness: Endianness, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let mut field_4 = [0; 2];
    let field_1 = endianness.read_u16(&mut reader)?;
    let field_2 = endianness.read_u16(&mut reader)?;
    let field_3 = endianness.read_u16(&mut reader)?;
    reader.read_exact(&mut field_4[..])?;

    Ok(Control1_4 {
      field_1,
      field_2,
      field_3,
      field_4,
    })
  }
}
