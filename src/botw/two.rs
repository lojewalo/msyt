use crate::{
  Result,
  botw::{MainControl, SubControl},
};

use byteordered::Endian;

use failure::ResultExt;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

mod one_field;
mod variable;

use self::{
  one_field::Control2OneField,
  variable::Control2Variable,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Control2 {
  Type1(Control2Variable),
  Type2(Control2Variable),
  Type3(Control2OneField),
  Type4(Control2OneField),
  Type7(Control2OneField),
  Type8(Control2OneField),
  Type9(Control2Variable),
  Type10(Control2OneField),
  Type11(Control2Variable),
  Type13(Control2OneField),
  Type14(Control2Variable),
  Type15(Control2Variable),
  Type16(Control2Variable),
  Type17(Control2Variable),
  Type18(Control2Variable),
  Type19(Control2Variable),
}

impl MainControl for Control2 {
  fn marker(&self) -> u16 {
    2
  }

  fn parse(header: &Header, buf: &[u8]) -> Result<(usize, Self)> {
    let mut c = Cursor::new(buf);

    let kind = header.endianness().read_u16(&mut c)?;
    let control = match kind {
      1 => Control2::Type1(Control2Variable::parse(header, &mut c).with_context(|_| "could not parse control subtype 1")?),
      2 => Control2::Type2(Control2Variable::parse(header, &mut c).with_context(|_| "could not parse control subtype 2")?),
      3 => Control2::Type3(Control2OneField::parse(header, &mut c).with_context(|_| "could not parse control subtype 3")?),
      4 => Control2::Type4(Control2OneField::parse(header, &mut c).with_context(|_| "could not parse control subtype 4")?),
      7 => Control2::Type7(Control2OneField::parse(header, &mut c).with_context(|_| "could not parse control subtype 7")?),
      8 => Control2::Type8(Control2OneField::parse(header, &mut c).with_context(|_| "could not parse control subtype 8")?),
      9 => Control2::Type9(Control2Variable::parse(header, &mut c).with_context(|_| "could not parse control subtype 9")?),
      10 => Control2::Type10(Control2OneField::parse(header, &mut c).with_context(|_| "could not parse control subtype 10")?),
      11 => Control2::Type11(Control2Variable::parse(header, &mut c).with_context(|_| "could not parse control subtype 11")?),
      13 => Control2::Type13(Control2OneField::parse(header, &mut c).with_context(|_| "could not parse control subtype 13")?),
      14 => Control2::Type14(Control2Variable::parse(header, &mut c).with_context(|_| "could not parse control subtype 14")?),
      15 => Control2::Type15(Control2Variable::parse(header, &mut c).with_context(|_| "could not parse control subtype 15")?),
      16 => Control2::Type16(Control2Variable::parse(header, &mut c).with_context(|_| "could not parse control subtype 16")?),
      17 => Control2::Type17(Control2Variable::parse(header, &mut c).with_context(|_| "could not parse control subtype 17")?),
      18 => Control2::Type18(Control2Variable::parse(header, &mut c).with_context(|_| "could not parse control subtype 18")?),
      19 => Control2::Type19(Control2Variable::parse(header, &mut c).with_context(|_| "could not parse control subtype 19")?),
      x => failure::bail!("unknown control 2 type: {}", x),
    };

    Ok((
      c.position() as usize,
      control,
    ))
  }

  fn write(&self, header: &Header, mut writer: &mut Write) -> Result<()> {
    let (marker, sub) = match *self {
      Control2::Type1(ref c) => (1, c as &SubControl),
      Control2::Type2(ref c) => (2, c as &SubControl),
      Control2::Type3(ref c) => (3, c as &SubControl),
      Control2::Type4(ref c) => (4, c as &SubControl),
      Control2::Type7(ref c) => (7, c as &SubControl),
      Control2::Type8(ref c) => (8, c as &SubControl),
      Control2::Type9(ref c) => (9, c as &SubControl),
      Control2::Type10(ref c) => (10, c as &SubControl),
      Control2::Type11(ref c) => (11, c as &SubControl),
      Control2::Type13(ref c) => (13, c as &SubControl),
      Control2::Type14(ref c) => (14, c as &SubControl),
      Control2::Type15(ref c) => (15, c as &SubControl),
      Control2::Type16(ref c) => (16, c as &SubControl),
      Control2::Type17(ref c) => (17, c as &SubControl),
      Control2::Type18(ref c) => (18, c as &SubControl),
      Control2::Type19(ref c) => (19, c as &SubControl),
    };

    header.endianness().write_u16(&mut writer, marker)
      .with_context(|_| format!("could not write marker for subtype {}", marker))?;
    sub.write(header, &mut writer)
      .with_context(|_| format!("could not write subtype {}", marker))
      .map_err(Into::into)
  }
}
