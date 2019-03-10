use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct Control201 {
  pub field_1: u16,
  pub field_2: u16,
  pub field_3: u16,
  pub field_4: u16,
  pub field_5: u16,
  pub field_6: u16,
  pub field_7: String,
}

impl Control201 {
  pub fn parse(endianness: Endianness, buf: &[u8]) -> Result<(usize, Control201)> {
    let mut c = Cursor::new(buf);
    let field_1 = endianness.read_u16(&mut c)?;
    let field_2 = endianness.read_u16(&mut c)?;
    let field_3 = endianness.read_u16(&mut c)?;
    let field_4 = endianness.read_u16(&mut c)?;
    let field_5 = endianness.read_u16(&mut c)?;
    let field_6 = endianness.read_u16(&mut c)?;

    let field_7_len = endianness.read_u16(&mut c)?;
    let mut str_buf = vec![0; field_7_len as usize];
    c.read_exact(&mut str_buf)?;

    // FIXME: encoding
    let utf16_str: Vec<u16> = str_buf.chunks(2)
      .map(|bs| endianness.read_u16(bs).map_err(Into::into))
      .collect::<Result<_>>()?;
    let field_7 = String::from_utf16(&utf16_str)?;

    Ok((
      c.position() as usize,
      Control201 {
        field_1,
        field_2,
        field_3,
        field_4,
        field_5,
        field_6,
        field_7,
      }
    ))
  }
}
