use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control4_0 {
  field_1: u16,
  string: String,
}

impl Control4_0 {
  pub(crate) fn parse(endianness: Endianness, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let field_1 = endianness.read_u16(&mut reader)?;
    let str_len = endianness.read_u16(&mut reader)?;

    let mut str_bytes = vec![0; str_len as usize];
    reader.read_exact(&mut str_bytes)?;

    // FIXME: handle encoding
    let utf16_str: Vec<u16> = str_bytes.chunks(2)
      .map(|bs| endianness.read_u16(bs).map_err(Into::into))
      .collect::<Result<_>>()?;
    let string = String::from_utf16(&utf16_str)?;

    Ok(Control4_0 {
      field_1,
      string,
    })
  }
}
