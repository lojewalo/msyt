# msyt

A human readable and editable format for MSBT files.

msyt is a YAML format specification for MSBT files, allowing easy reading and writing. This repo
houses the importer and exporter for MSYT files.

## Usage

### Importing

`msyt import path/to/file.msyt another/path/to/file.msyt`

Currently, the MSBT files to import into must be in the same directory (i.e. adjacent) to the MSYT
files provided. The output will have the extension `.msbt-new`, but this will be changed soon.

### Exporting

`msyt export path/to/file.msbt a/different/file.msbt`

MSYT files will be output in the same directory as the MSBT files.

## Building

```shell
# from repo root
cargo build

# with optimisations
cargo build --release
```
