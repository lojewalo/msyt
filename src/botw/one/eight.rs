use crate::Result;

use byteordered::Endian;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read, Write};

const UNKNOWN: [u8; 4] = [255, 255, 0, 0];

#[derive(Debug, Deserialize, Serialize)]
pub struct Control1_8 {
  unknown_1: Vec<[u8; 4]>,
  field_1: Vec<u16>,
  field_2: [u8; 4],
}

impl Control1_8 {
  pub(crate) fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
    let len = header.endianness().read_u16(&mut reader)?;
    let mut buf = vec![0; len as usize - 4];
    reader.read_exact(&mut buf)?;

    let mut unknown_count = 0;
    for (i, unknown) in buf.chunks(4).map(|x| x == &UNKNOWN[..]).enumerate() {
      if i > 0 && unknown_count == 0 {
        break;
      }
      if !unknown && unknown_count > 0 {
        break;
      }
      if unknown {
        unknown_count += 1;
      }
    }
    let unknown_1 = (0..unknown_count).map(|_| UNKNOWN).collect();

    let field_1 = buf[unknown_count * 4..]
      .chunks(2)
      .map(|bs| header.endianness().read_u16(bs).map_err(Into::into))
      .collect::<Result<_>>()?;

    let mut field_2 = [0; 4];
    reader.read_exact(&mut field_2[..])?;

    Ok(Control1_8 {
      unknown_1,
      field_1,
      field_2,
    })
  }

  pub(crate) fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    let len = self.unknown_1.len() * UNKNOWN.len()
      + self.field_1.len() * 2
      + self.field_2.len();
    header.endianness().write_u16(&mut writer, len as u16)?;

    for unknown in &self.unknown_1 {
      writer.write_all(&unknown[..])?;
    }

    for &byte in &self.field_1 {
      header.endianness().write_u16(&mut writer, byte)?;
    }

    writer.write_all(&self.field_2[..])?;

    Ok(())
  }
}
