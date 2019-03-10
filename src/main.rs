use clap::ArgMatches;
use failure::Fail;
use indexmap::IndexMap;
use msbt::Msbt;
use rayon::prelude::*;
use walkdir::WalkDir;

use std::{
  fs::File,
  io::{BufReader, BufWriter},
  path::PathBuf,
};

mod botw;
mod cli;
mod model;

use self::model::{Msyt, Content};

pub type Result<T> = std::result::Result<T, failure::Error>;

fn main() {
  std::process::exit(match inner() {
    Ok(()) => 0,
    Err(e) => {
      eprintln!("an error occurred - see below for details");
      eprintln!();
      eprintln!("{}", e);
      for (indent, err) in e.iter_causes().enumerate() {
        let indent_str: String = std::iter::repeat("  ").take(indent + 1).collect();
        eprintln!("{}{}", indent_str, err);
      }
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

  paths
    .into_par_iter()
    .map(|path| {
      let msyt_file = File::open(&path)?;
      let msyt: Msyt = serde_yaml::from_reader(BufReader::new(msyt_file))?;

      let lossy_path = path.to_string_lossy();
      let base_path = match lossy_path.rsplitn(2, '.').nth(1) {
        Some(b) => b,
        None => failure::bail!("invalid path (no extension): {}", lossy_path),
      };

      let msbt_path = format!("{}.msbt", base_path);
      let msbt_file = File::open(msbt_path)?;

      let mut msbt = Msbt::from_reader(BufReader::new(msbt_file))?;

      for (key, contents) in msyt.entries {
        let new_val = Content::write_all(msbt.header(), &contents)?;
        if let Some(ref mut lbl1) = msbt.lbl1_mut() {
          if let Some(label) = lbl1.labels_mut().iter_mut().find(|x| x.name() == key) {
            if let Err(()) = label.set_value_raw(new_val) {
              failure::bail!("could not set raw string at index {}", label.index());
            }
          }
        }
      }

      let new_msbt = File::create(format!("{}.msbt-new", base_path))?;
      msbt.write_to(BufWriter::new(new_msbt))?;

      Ok(())
    })
    .collect::<Result<_>>()
}

fn export(matches: &ArgMatches) -> Result<()> {
  let paths: Vec<PathBuf> = if matches.is_present("dir_mode") {
    find_files(matches.values_of("paths").expect("required argument"), "msbt")?
  } else {
    matches.values_of("paths").expect("required argument").map(PathBuf::from).collect()
  };

  paths
    .into_par_iter()
    .map(|path| {
      let msbt_file = File::open(&path)?;
      let msbt = Msbt::from_reader(BufReader::new(msbt_file))?;

      let lbl1 = match msbt.lbl1() {
        Some(lbl) => lbl,
        None => failure::bail!("invalid msbt: missing lbl1: {}", path.to_string_lossy()),
      };

      let mut entries = IndexMap::with_capacity(lbl1.labels().len());

      for label in lbl1.labels() {
        let mut all_content = Vec::new();

        let raw_value = label.value_raw()
          .ok_or_else(|| failure::format_err!(
            "invalid msbt at {}: missing string for label {}",
            path.to_string_lossy(),
            label.name(),
          ))?;
        let mut parts = self::botw::parse_controls(msbt.header(), raw_value)
          .map_err(|e| e.context(format!("could not parse control sequences in {}", path.to_string_lossy())))?;
        all_content.append(&mut parts);
        entries.insert(label.name().to_string(), all_content);
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
      ).map_err(|e| e.context("could not write yaml to file"))?;

      Ok(())
    })
    .collect::<Result<_>>()
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
