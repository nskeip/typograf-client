# Typograf console client

Yet another console client for https://www.artlebedev.ru/typograf/

## Usage and flags

*IMPORTANT:* the default values of options (taken from API clients examples) seemed not convenient to me,
so they are not exactly the same, but should work well for Markdown and texts that are post-processed with other tools.

```
USAGE:
    typograf-client [FLAGS] [OPTIONS] <input>

FLAGS:
    -h, --help                 Prints help information
    -i, --inplace              Edit the file inplace
    -s, --skip-front-matter    Skip front matter header
    -V, --version              Prints version information

OPTIONS:
        --encoding <encoding>          Input encoding [default: UTF-8]
        --entity-type <entity-type>    *Not sure how it works, but 4 is okay*: switches xml, mixed or something
                                       [default: 4]
        --max-no-br <max-no-br>        *Don't know what it is*, but default is 3 [default: 3]
        --use-br <use-br>              Use <br /> for multiline text: 1 is "yes" [default: 1]
        --use-p <use-p>                Use <p> for multiline text: 1 is "yes" [default: 1]

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
