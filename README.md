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
OLEkit 1.0.1
A simple command line tool for working with OLE file

USAGE:
    olekit [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    cat     Concatenate entries and print on the standard output
    help    Prints this message or the help of the given subcommand(s)
    ls      List OLE file entries
```

### How to list entries: `ls` command

```bash
$ olekit help ls
olekit-ls
List OLE file entries

USAGE:
    olekit ls [FLAGS] <FILE>

FLAGS:
        --color             colorize the output
    -d, --details           print more details for each entry
    -f, --full-path         print the full path of each entry
        --help              Prints help information
    -h, --human-readable    with -l and -s, print sizes like 1K 234M 2G etc.
    -i, --idirid            print the index number of each file
    -s, --size              print the allocated size of each file, in blocks
    -V, --version           Prints version information

ARGS:
    <FILE>    OLE file to analyze
```

### How to extract entry content: `cat` command

```bash
$ olekit help cat
olekit-cat
Concatenate entries and print on the standard output

USAGE:
    olekit cat <FILE> <ID>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>     OLE file to analyze
    <ID>...    IDs of the entries
```

## License

<http://www.wtfpl.net/about/>
