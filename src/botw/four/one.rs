use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control4_1 {
  field_1: Vec<u8>,
}

impl Control4_1 {
  pub(crate) fn parse(endianness: Endianness, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let field_1_len = endianness.read_u16(&mut reader)?;
    let mut field_1 = vec![0; field_1_len as usize];
    reader.read_exact(&mut field_1)?;

    Ok(Control4_1 {
      field_1,
    })
  }
}
