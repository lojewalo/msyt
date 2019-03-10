use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::Cursor;

#[derive(Debug, Deserialize, Serialize)]
pub struct Control1_3 {
  field_1: u16,
  field_2: u32,
}

impl Control1_3 {
  pub(crate) fn parse(endianness: Endianness, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    Ok(Control1_3 {
      field_1: endianness.read_u16(&mut reader)?,
      field_2: endianness.read_u32(&mut reader)?,
    })
  }
}
