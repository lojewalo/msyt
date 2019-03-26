use clap::ArgMatches;
use failure::Fail;
use indexmap::IndexMap;
use msbt::{Msbt, section::Atr1};
use rayon::prelude::*;

use std::{
  fs::File,
  io::{BufReader, BufWriter, Write},
  path::{Path, PathBuf},
};

use crate::{
  Result,
  model::{Msyt, Entry},
  subcommand::find_files,
};

pub fn export(matches: &ArgMatches) -> Result<()> {
  let input_paths: Vec<&str> = matches.values_of("paths").expect("required clap arg").collect();
  let paths: Vec<PathBuf> = if matches.is_present("dir_mode") {
    find_files(input_paths.iter().map(Clone::clone), "msbt")?
  } else {
    input_paths.iter().map(PathBuf::from).collect()
  };
  let output_path = matches.value_of("output").map(Path::new);

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
        let mut parts = crate::botw::parse_controls(msbt.header(), raw_value)
          .map_err(|e| e.context(format!("could not parse control sequences in {}", path.to_string_lossy())))?;
        all_content.append(&mut parts);
        let entry = Entry {
          attributes: msbt.atr1()
            .and_then(|a| a.strings()
              .get(label.index() as usize)
              .map(|s| crate::util::strip_nul(*s))
              .map(ToString::to_string)),
          contents: all_content,
        };
        entries.insert(label.name().to_string(), entry);
      }

      entries.sort_keys();

      let msyt = Msyt {
        entries,
        group_count: lbl1.group_count(),
        atr1_unknown: msbt.atr1().map(Atr1::unknown_1),
        ato1: msbt.ato1().map(|a| a.unknown_bytes().to_vec()),
        tsy1: msbt.tsy1().map(|a| a.unknown_bytes().to_vec()),
        nli1: msbt.nli1().map(|a| crate::model::Nli1 {
          id_count: a.id_count(),
          global_ids: a.global_ids().clone(),
        }),
      };

      let dest = match output_path {
        Some(output) => {
          let stripped_path = match input_paths.iter().flat_map(|input| path.strip_prefix(input)).next() {
            Some(s) => s,
            None => failure::bail!("no input path works as a prefix on {}", path.to_string_lossy()),
          };
          output.join(stripped_path).with_extension("msyt")
        },
        None => path.with_extension("msyt"),
      };
      if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
      }
      let mut writer = BufWriter::new(File::create(dest)?);
      serde_yaml::to_writer(
        &mut writer,
        &msyt,
      ).map_err(|e| e.context("could not write yaml to file"))?;
      // add final newline
      writer.write_all(b"\n").map_err(|e| e.context("could not write final newline to file"))?;

      Ok(())
    })
    .collect::<Result<_>>()
}
