use crate::{
  Result,
  botw::{MainControl, SubControl},
};

use byteordered::Endian;

use failure::ResultExt;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

mod zero;
mod one;
mod two;
mod three;

use self::{
  zero::Control4_0,
  one::Control4_1,
  two::Control4_2,
  three::Control4_3,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Control4 {
  Type0(Control4_0),
  Type1(Control4_1),
  Type2(Control4_2),
  Type3(Control4_3),
}

impl MainControl for Control4 {
  fn marker(&self) -> u16 {
    4
  }

  fn parse(header: &Header, buf: &[u8]) -> Result<(usize, Self)> {
    let mut c = Cursor::new(buf);

    let kind = header.endianness().read_u16(&mut c).with_context(|_| "could not read control subtype marker")?;
    let control = match kind {
      0 => Control4::Type0(Control4_0::parse(header, &mut c).with_context(|_| "could not parse control subtype 0")?),
      1 => Control4::Type1(Control4_1::parse(header, &mut c).with_context(|_| "could not parse control subtype 1")?),
      2 => Control4::Type2(Control4_2::parse(header, &mut c).with_context(|_| "could not parse control subtype 2")?),
      3 => Control4::Type3(Control4_3::parse(header, &mut c).with_context(|_| "could not parse control subtype 3")?),
      x => failure::bail!("unknown control 4 subtype: {}", x),
    };

    Ok((
      c.position() as usize,
      control,
    ))
  }

  fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    let sub = match *self {
      Control4::Type0(ref c) => c as &SubControl,
      Control4::Type1(ref c) => c as &SubControl,
      Control4::Type2(ref c) => c as &SubControl,
      Control4::Type3(ref c) => c as &SubControl,
    };
    header.endianness().write_u16(&mut writer, sub.marker())
      .with_context(|_| format!("could not write control subtype marker {}", sub.marker()))?;
    sub.write(header, &mut writer)
      .with_context(|_| format!("could not write control subtype {}", sub.marker()))
      .map_err(Into::into)
  }
}
