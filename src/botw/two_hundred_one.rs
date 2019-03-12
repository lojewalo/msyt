use crate::{
  Result,
  botw::{Control, MainControl, RawControl},
};

use byteordered::Endian;

use failure::ResultExt;

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

impl MainControl for Control201 {
  fn marker(&self) -> u16 {
    201
  }

  fn parse(header: &Header, buf: &[u8]) -> Result<(usize, Control)> {
    let mut c = Cursor::new(buf);
    let field_1 = header.endianness().read_u16(&mut c).with_context(|_| "could not read field_1")?;
    let field_2 = header.endianness().read_u16(&mut c).with_context(|_| "could not read field_2")?;
    let field_3 = header.endianness().read_u16(&mut c).with_context(|_| "could not read field_3")?;
    let field_4 = header.endianness().read_u16(&mut c).with_context(|_| "could not read field_4")?;
    let field_5 = header.endianness().read_u16(&mut c).with_context(|_| "could not read field_5")?;
    let field_6 = header.endianness().read_u16(&mut c).with_context(|_| "could not read field_6")?;

    let field_7_len = header.endianness().read_u16(&mut c).with_context(|_| "could not read field_7 length")?;
    let mut str_buf = vec![0; field_7_len as usize];
    c.read_exact(&mut str_buf).with_context(|_| "could not read field_7")?;

    let field_7 = match header.encoding() {
      Encoding::Utf16 => {
        let utf16_str: Vec<u16> = str_buf.chunks(2)
          .map(|bs| header.endianness().read_u16(bs).map_err(Into::into))
          .collect::<Result<_>>()
          .with_context(|_| "could not read u16s from string bytes")?;
        String::from_utf16(&utf16_str).with_context(|_| "could not parse utf-16 string")?
      },
      Encoding::Utf8 => String::from_utf8(str_buf).with_context(|_| "could not parse utf-8 string")?,
    };

    Ok((
      c.position() as usize,
      Control::Raw(RawControl::TwoHundredOne(Control201 {
        field_1,
        field_2,
        field_3,
        field_4,
        field_5,
        field_6,
        field_7,
      }))
    ))
  }

  fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    header.endianness().write_u16(&mut writer, self.field_1).with_context(|_| "could not write field_1")?;
    header.endianness().write_u16(&mut writer, self.field_2).with_context(|_| "could not write field_2")?;
    header.endianness().write_u16(&mut writer, self.field_3).with_context(|_| "could not write field_3")?;
    header.endianness().write_u16(&mut writer, self.field_4).with_context(|_| "could not write field_4")?;
    header.endianness().write_u16(&mut writer, self.field_5).with_context(|_| "could not write field_5")?;
    header.endianness().write_u16(&mut writer, self.field_6).with_context(|_| "could not write field_6")?;

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

    header.endianness().write_u16(&mut writer, str_bytes.len() as u16)
      .with_context(|_| "could not write field_7 length")?;
    writer.write_all(&str_bytes).with_context(|_| "could not write field 7")?;

    Ok(())
  }
}
