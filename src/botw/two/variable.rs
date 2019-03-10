use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control2Variable {
  field_1: u16,
  string: String,
  field_3: u16,
}

impl Control2Variable {
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

    let field_3 = endianness.read_u16(&mut reader)?;

    Ok(Control2Variable {
      field_1,
      string,
      field_3,
    })
  }
}
