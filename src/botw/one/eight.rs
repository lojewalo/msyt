use crate::Result;

use byteordered::{Endianness, Endian};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read};

const UNKNOWN: [u8; 4] = [255, 255, 0, 0];

#[derive(Debug, Deserialize, Serialize)]
pub struct Control1_8 {
  unknown_1: Vec<[u8; 4]>,
  field_1: Vec<u16>,
  field_2: [u8; 4],
}

impl Control1_8 {
  pub(crate) fn parse(endianness: Endianness, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let len = endianness.read_u16(&mut reader)?;
    let mut buf = vec![0; len as usize - 4];
    reader.read_exact(&mut buf)?;

    let unknowns = buf.chunks(4).filter(|&x| x == &UNKNOWN[..]).count();
    let unknown_1 = (0..unknowns).map(|_| UNKNOWN).collect();

    let field_1 = buf[unknowns * 4..]
      .chunks(2)
      .map(|bs| endianness.read_u16(bs).map_err(Into::into))
      .collect::<Result<_>>()?;

    let mut field_2 = [0; 4];
    reader.read_exact(&mut field_2[..])?;

    Ok(Control1_8 {
      unknown_1,
      field_1,
      field_2,
    })
  }
}
