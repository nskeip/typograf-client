# Typograf console client

Yet another console client for https://www.artlebedev.ru/typograf/

## Usage and flags

```
USAGE:
    typograf-client [FLAGS] <input>

FLAGS:
    -h, --help                 Prints help information
    -i, --inplace              Edit the file inplace
    -s, --skip-front-matter    Skip front matter header
    -V, --version              Prints version information

ARGS:
    <input>    Input file
```

### `-h`

It just says it helps ^_^.

### `-i` aka inplace

Does not display the contents of the file, but re-writes it (so better use with caution).

### `-s` aka skip-front-matter

If you use [Zola](https://www.getzola.org/), than you add some metadata to your MD-files.
This flag prevents metadata from being formatted.
