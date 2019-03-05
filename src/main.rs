use clap::ArgMatches;
use indexmap::IndexMap;
use itertools::Itertools;
use msbt::{Msbt, Encoding};
use walkdir::WalkDir;

use std::{
  fs::File,
  io::{BufReader, BufWriter},
  path::PathBuf,
};

mod cli;
mod model;

use self::model::{Msyt, Content};

pub type Result<T> = std::result::Result<T, failure::Error>;

fn main() {
  std::process::exit(match inner() {
    Ok(()) => 0,
    Err(e) => {
      eprintln!("error: {}", e);
      1
    },
  });
}

fn inner() -> Result<()> {
  let matches = self::cli::app().get_matches();

  match matches.subcommand() {
    ("export", Some(sub_matches)) => export(sub_matches),
    ("import", Some(sub_matches)) => import(sub_matches),
    _ => unreachable!("clap allowed an unspecified subcommand"),
  }
}

fn import(matches: &ArgMatches) -> Result<()> {
  let paths: Vec<PathBuf> = if matches.is_present("dir_mode") {
    find_files(matches.values_of("paths").expect("required argument"), "msyt")?
  } else {
    matches.values_of("paths").expect("required argument").map(PathBuf::from).collect()
  };

  for path in paths {
    let msyt_file = File::open(&path)?;
    let msyt: Msyt = serde_yaml::from_reader(msyt_file)?;

    let lossy_path = path.to_string_lossy();
    let base_path = match lossy_path.rsplitn(2, '.').nth(1) {
      Some(b) => b,
      None => failure::bail!("invalid path (no extension): {}", lossy_path),
    };

    let msbt_path = format!("{}.msbt", base_path);
    let msbt_file = File::open(msbt_path)?;

    let mut msbt = Msbt::from_reader(BufReader::new(msbt_file))?;

    for (key, contents) in msyt.entries {
      if let Some(ref mut lbl1) = msbt.lbl1 {
        if let Some(mut label) = lbl1.labels.iter_mut().find(|x| x.name == key) {
          let new_val = match msbt.header.encoding {
            Encoding::Utf16 => String::from_utf16(&Content::combine_utf16(&contents))?,
            Encoding::Utf8 => String::from_utf8(Content::combine_utf8(&contents))?,
          };
          label.value = new_val.clone();

          if let Some(ref mut txt2) = msbt.txt2 {
            txt2.strings[label.index as usize] = new_val;
          }
        }
      }
    }

    let new_msbt = File::create(format!("{}.msbt-new", base_path))?;
    msbt.write_to(BufWriter::new(new_msbt))?;
  }

  Ok(())
}

fn export(matches: &ArgMatches) -> Result<()> {
  let paths: Vec<PathBuf> = if matches.is_present("dir_mode") {
    find_files(matches.values_of("paths").expect("required argument"), "msbt")?
  } else {
    matches.values_of("paths").expect("required argument").map(PathBuf::from).collect()
  };

  for path in paths {
    let msbt_file = File::open(&path)?;
    let msbt = Msbt::from_reader(BufReader::new(msbt_file))?;

    let lbl1 = match msbt.lbl1 {
      Some(lbl) => lbl,
      None => failure::bail!("Invalid MSBT file (missing LBL1): {}", path.to_string_lossy()),
    };

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
              Content::Text(String::from_utf16(&bytes)?)
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
              Content::Text(String::from_utf8(bytes)?)
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

    let lossy_path = path.to_string_lossy();
    let base = match lossy_path.rsplitn(2, '.').nth(1) {
      Some(b) => b,
      None => failure::bail!("invalid path (no extension): {}", path.to_string_lossy()),
    };
    serde_yaml::to_writer(
      BufWriter::new(File::create(format!("{}.msyt", base))?),
      &msyt,
    )?;
  }

  Ok(())
}

fn find_files<'a>(paths: impl Iterator<Item = &'a str>, ext: &str) -> Result<Vec<PathBuf>> {
  paths
    .flat_map(|p| WalkDir::new(p)
      .into_iter()
      .map(|e| e.map(walkdir::DirEntry::into_path))
      .filter(|p| p.as_ref().map(|p| p.is_file() && p.extension().and_then(std::ffi::OsStr::to_str) == Some(ext)).unwrap_or(false)))
      .map(|p| p.map_err(Into::into))
    .collect()
}
