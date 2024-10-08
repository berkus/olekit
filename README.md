olekit
===============

`olekit` is a command line tool for analyzing Microsoft Compound Document files.

## Installation

Using [cargo](https://crates.io/):

```bash
cargo install olekit
```

## Usage

```bash
$ olekit --help
Usage: olekit <command> [<args>]

OLEkit.

Options:
  --help            display usage information

Commands:
  ls                List OLE file entries.
  cat               Concatenate entries and print on the standard output.
```

### How to list entries: `ls` command

```bash
$ olekit ls --help
Usage: olekit ls <file> [-h] [-s] [-c] [-d] [-i] [-f]

List OLE file entries.

Positional Arguments:
  file              an OLE file to analyze

Options:
  -h, --human-readable
                    with -l and --size, print sizes like 1K 234M 2G etc.
  -s, --size        print the allocated size of each file, in blocks
  -c, --color       colorize the output
  -d, --details     print more details for each entry
  -i, --idirid      print the index number of each file
  -f, --full-path   print the full path of each entry
  --help            display usage information
```

### How to extract entry content: `cat` command

```bash
$ olekit cat --help
Usage: olekit cat <file> [<ids...>]

Concatenate entries and print on the standard output.

Positional Arguments:
  file              an OLE file to analyze
  ids               IDs of the entries

Options:
  --help            display usage information
```

## License

<http://www.wtfpl.net/about/>
