use crate::Result;

use byteordered::Endian;

use msbt::{Encoding, Header};

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read, Write};

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
  pub fn parse(header: &Header, buf: &[u8]) -> Result<(usize, Control201)> {
    let mut c = Cursor::new(buf);
    let field_1 = header.endianness().read_u16(&mut c)?;
    let field_2 = header.endianness().read_u16(&mut c)?;
    let field_3 = header.endianness().read_u16(&mut c)?;
    let field_4 = header.endianness().read_u16(&mut c)?;
    let field_5 = header.endianness().read_u16(&mut c)?;
    let field_6 = header.endianness().read_u16(&mut c)?;

    let field_7_len = header.endianness().read_u16(&mut c)?;
    let mut str_buf = vec![0; field_7_len as usize];
    c.read_exact(&mut str_buf)?;

    let field_7 = match header.encoding() {
      Encoding::Utf16 => {
        let utf16_str: Vec<u16> = str_buf.chunks(2)
          .map(|bs| header.endianness().read_u16(bs).map_err(Into::into))
          .collect::<Result<_>>()?;
        String::from_utf16(&utf16_str)?
      },
      Encoding::Utf8 => String::from_utf8(str_buf)?,
    };

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

  pub fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1)?;
    header.endianness().write_u16(&mut writer, self.field_2)?;
    header.endianness().write_u16(&mut writer, self.field_3)?;
    header.endianness().write_u16(&mut writer, self.field_4)?;
    header.endianness().write_u16(&mut writer, self.field_5)?;
    header.endianness().write_u16(&mut writer, self.field_6)?;

    let str_bytes = match header.encoding() {
      Encoding::Utf16 => {
        let mut buf = [0; 2];
        self.field_7.encode_utf16()
          .flat_map(|x| {
            header.endianness().write_u16(&mut buf[..], x).expect("failed to write to array");
            buf.to_vec()
          })
          .collect()
      },
      Encoding::Utf8 => self.field_7.as_bytes().to_vec(),
    };

    header.endianness().write_u16(&mut writer, str_bytes.len() as u16)?;
    writer.write_all(&str_bytes)?;

    Ok(())
  }
}
