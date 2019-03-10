use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::Cursor;

#[derive(Debug, Deserialize, Serialize)]
pub struct Control0_0 {
  field_1: u16,
  field_2: u16,
  field_3: u16,
}

impl Control0_0 {
  pub(crate) fn parse(endianness: Endianness, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    Ok(Control0_0 {
      field_1: endianness.read_u16(&mut reader)?,
      field_2: endianness.read_u16(&mut reader)?,
      field_3: endianness.read_u16(&mut reader)?,
    })
  }
}
