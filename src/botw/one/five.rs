use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control1_5 {
  field_1: u16,
  field_2: u16,
  field_3: u16,
  field_4: u16,
  field_5: [u8; 2],
}

impl Control1_5 {
  pub(crate) fn parse(endianness: Endianness, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let mut field_5 = [0; 2];
    let field_1 = endianness.read_u16(&mut reader)?;
    let field_2 = endianness.read_u16(&mut reader)?;
    let field_3 = endianness.read_u16(&mut reader)?;
    let field_4 = endianness.read_u16(&mut reader)?;
    reader.read_exact(&mut field_5[..])?;

    Ok(Control1_5 {
      field_1,
      field_2,
      field_3,
      field_4,
      field_5,
    })
  }
}
