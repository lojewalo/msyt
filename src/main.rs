use itertools::Itertools;
use msbt::{Msbt, Encoding};
use serde_derive::{Deserialize, Serialize};
use indexmap::IndexMap;

use std::{
  fs::File,
  io::{BufReader, BufWriter},
};

fn main() {
  let sub = std::env::args().nth(1).unwrap();
  if sub == "export" {
    export();
  } else if sub == "import" {
    import();
  } else {
    eprintln!("first arg must be import or export");
  }
}

fn import() {
  for path in std::env::args().skip(2) {
    let msyt_file = File::open(&path).unwrap();
    let msyt: Msyt = serde_yaml::from_reader(msyt_file).unwrap();

    let base_path = path.rsplitn(2, '.').nth(1).unwrap();

    let msbt_path = format!("{}.msbt", base_path);
    let msbt_file = File::open(msbt_path).unwrap();

    let mut msbt = Msbt::from_reader(BufReader::new(msbt_file)).unwrap();

    for (key, contents) in msyt.entries {
      if let Some(ref mut lbl1) = msbt.lbl1 {
        if let Some(mut label) = lbl1.labels.iter_mut().find(|x| x.name == key) {
          let new_val = match msbt.header.encoding {
            Encoding::Utf16 => String::from_utf16(&combine_contents_utf16(&contents)).unwrap(),
            Encoding::Utf8 => unsafe { String::from_utf8_unchecked(combine_contents_utf8(&contents)) },
          };
          label.value = new_val.clone();

          if let Some(ref mut txt2) = msbt.txt2 {
            txt2.strings[label.index as usize] = new_val;
          }
        }
      }
    }

    let new_msbt = File::create(format!("{}.msbt-new", base_path)).unwrap();
    msbt.write_to(BufWriter::new(new_msbt)).unwrap();
  }
}

fn export() {
  for path in std::env::args().skip(2) {
    let msbt_file = File::open(&path).unwrap();
    let msbt = Msbt::from_reader(BufReader::new(msbt_file)).unwrap();

    let lbl1 = msbt.lbl1.unwrap();

    let mut entries = IndexMap::with_capacity(lbl1.labels.len());

    for label in lbl1.labels {
      let mut all_content = Vec::new();

      match msbt.header.encoding {
        Encoding::Utf16 => {
          let grouped = label.value
            .encode_utf16()
            .group_by(|&x| x < 255 && (x as u8).is_ascii_alphanumeric() || (x as u8).is_ascii_punctuation() || (x as u8).is_ascii_whitespace());
          for (is_ascii, part) in &grouped {
            let bytes: Vec<u16> = part.collect();
            let content = if is_ascii {
              Content::Ascii(String::from_utf16(&bytes).unwrap())
            } else {
              Content::Utf16Bytes(bytes)
            };
            all_content.push(content);
          }
        },
        Encoding::Utf8 => {
          let grouped = label.value
            .as_bytes()
            .iter()
            .group_by(|&x| x.is_ascii_alphanumeric() || x.is_ascii_punctuation() || x.is_ascii_whitespace());
          for (is_ascii, part) in &grouped {
            let bytes: Vec<u8> = part.cloned().collect();
            let content = if is_ascii {
              Content::Ascii(unsafe { String::from_utf8_unchecked(bytes) })
            } else {
              Content::Utf8Bytes(bytes)
            };
            all_content.push(content);
          }
        }
      }

      entries.insert(label.name, all_content);
    }

    let msyt = Msyt { entries };

    let base = path.rsplitn(2, '.').nth(1).unwrap();
    serde_yaml::to_writer(
      BufWriter::new(File::create(format!("{}.msyt", base)).unwrap()),
      &msyt,
    ).unwrap();
  }
}

#[derive(Debug, Deserialize, Serialize)]
struct Msyt {
  entries: IndexMap<String, Vec<Content>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Content {
  Ascii(String),
  Utf16Bytes(Vec<u16>),
  Utf8Bytes(Vec<u8>),
}

fn combine_contents_utf16(contents: &[Content]) -> Vec<u16> {
  let mut buf = Vec::new();

  for content in contents {
    match *content {
      Content::Ascii(ref s) => {
        let mut utf16_bytes: Vec<u16> = s.encode_utf16().collect();
        buf.append(&mut utf16_bytes);
      },
      Content::Utf16Bytes(ref b) => buf.append(&mut b.to_vec()),
      _ => panic!("utf8 bytes in utf16 file"),
    }
  }

  buf
}

fn combine_contents_utf8(contents: &[Content]) -> Vec<u8> {
  let mut buf = Vec::new();

  for content in contents {
    match *content {
      Content::Ascii(ref s) => buf.append(&mut s.as_bytes().to_vec()),
      Content::Utf8Bytes(ref b) => buf.append(&mut b.to_vec()),
      _ => panic!("utf16 bytes in utf8 file"),
    }
  }

  buf
}
