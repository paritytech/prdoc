# PRDoc

<figure>
<img src="https://github.com/paritytech/prdoc/actions/workflows/quick-check.yml/badge.svg?branch=master" alt="badge" />
</figure>

`prdoc` is a tool helping with `.prdoc` files. `.prdoc` files are YAML files following a defined schema and helping with
code change documentation. While platform like Github allow a simple description for a Pull Request (PR), this is
limited to a title, description and some labels. The description itself is often used to describe the change but not
document it.

The schema can be found here: [schema\_user.json](schema_user.json).

## Install

    cargo install prdoc

Alternatively, you may use a the container image:

        ENGINE=podman
        DOC_PATH="$PWD/tests/data/some"
        $ENGINE run --rm -it -v $DOC_PATH:/doc paritytech/prdoc --help
        $ENGINE run --rm -it -v $DOC_PATH:/doc paritytech/prdoc scan --all
        $ENGINE run --rm -it -v $DOC_PATH:/doc paritytech/prdoc check
        $ENGINE run --rm -it -v $DOC_PATH:/doc paritytech/prdoc load

The container image is working by default in `/doc` so it makes it simpler if you mount your doc there as shown
above.

## Features

-   provide the `prdoc` user schema

-   generate new documents

-   scan for `prdoc` in a folder

-   check `prdoc` files

-   load/aggregate `prdoc` files

## Schemas

### PR Doc

The documentation for PRs comes as a file with the extension `.prdoc`.
This is essentially a `yaml` file and the extension helps using the right JSON schema to validate the file.

In VScode, open your user settings and ensure you have the following section:

    "yaml.schemas":  {
        "/path/of/schema/schema_user.json": "*.prdoc"
    },

You also need:

    "files.associations": {
        "*.prdoc": "yaml",
    },

Should initially have created the file with another extension such as `.txt`, make sure to change the format to
`YAML` and the right schema should then be picked up.

### YAML Anchors

You may use YAML anchors as demonnstrated below.

    # Schema: Parity PR Documentation Schema (prdoc)

    # Anchoring is NOT supported yet.

    title: Foobar

    doc:
      - audience: Builder
        description: &desc |
          Sunt voluptate ad duis consequat ea in dolore non adipisicing incididunt
          ullamco enim qui enim.

      - audience: Validator
        description: *desc

    migrations:
      db: []
      runtime: []

    crates: []
    host_functions: []

## Usage

    prdoc is a cli utility to generate, check and load prdoc files.

    More at <https://github.com/paritytech/prdoc>

    Usage: prdoc [OPTIONS] [COMMAND]

    Commands:
      generate  Generate a new file. It will be printed to stdout by default unless you provide the `--save` flag
      check     Check one or MORE `prdoc` files for validity
      scan      Scan a directory for prdoc files based on their name
      load      Load one or more prdoc
      schema    Retrieve the JSON schema that is used internally
      help      Print this message or the help of the given subcommand(s)

    Options:
      -v, --version
              Show the version

      -j, --json
              Output as json

      -h, --help
              Print help (see a summary with '-h')

### Schema

    Retrieve the JSON schema that is used internally

    Usage: prdoc schema [OPTIONS]

    Options:
      -j, --json  Output as json
      -h, --help  Print help

### Generate

    Generate a new file. It will be printed to stdout by default unless you provide the `--save` flag

    Usage: prdoc generate [OPTIONS] <NUMBER>

    Arguments:
      <NUMBER>  Change number

    Options:
      -t, --title <TITLE>            Change title
      -s, --save                     Save the generated document to file with the proper naming
      -o, --output-dir <OUTPUT_DIR>  Output directory [default: .]
      -j, --json                     Output as json
      -h, --help                     Print help

### Scan

    Scan a directory for prdoc files based on their name

    Usage: prdoc scan [OPTIONS] [DIRECTORY]

    Arguments:
      [DIRECTORY]  directory path [default: .]

    Options:
      -a, --all   Also return invalid files
      -j, --json  Output as json
      -h, --help  Print help

### Check

    Check one or MORE `prdoc` files for validity

    Usage: prdoc check [OPTIONS]

    Options:
      -d, --directory <DIRECTORY>  Base directory for the files [default: .]
      -f, --file <FILE>            Directly specify the file to be checked. It can be relative to the base directory
      -n, --number <NUMBER>        number
      -l, --list <LIST>            Get the list of PR numbers from a file
      -j, --json                   Output as json
      -h, --help                   Print help

### Load

    Load one or more prdoc

    Usage: prdoc load [OPTIONS]

    Options:
      -d, --directory <DIRECTORY>  directory path [default: .]
      -f, --file <FILE>            file path
      -n, --number <NUMBER>        One or more PR numbers. Depending on the host OS, the max length of a command may differ. If you run into issues, make sure to check the `--list` option instead
      -l, --list <LIST>            Get the list of PR numbers from a file
      -j, --json                   Output as json
      -h, --help                   Print help

## Docker

If you prefer not having to install Rust & Cargo and have Docker installed, you may prefer to run a containerized
version of `prdoc`. The next chapters explain how to proceed.

### Run

Docker commands can end up quite lengthy so you may like to set an alias:

        alias prdoc='docker run --rm -it prdoc'

After setting this alias, you may use `prdoc` by simply invoking the `prdoc` command:

        prdoc --version

If you prefer a shorter a command, you may set an alias for `rl` instead of `prdoc`.

This is out of the scope of this documentation but note that you cannot just invoke `prdoc` check and expect it to work on
your local `specs.yaml`. For that to work, you need to mount your `specs.yaml` into the container. That looks like this:

        docker run --rm -it -v $PWD/specs.yaml:/usr/local/bin/specs.yaml <literal>prdoc</literal> list

### Build

You can pull the docker image from `paritytech`/`prdoc` or build you own:

        docker build -t prdoc .

## License

    Copyright 2021-2023 - Wilfried Kopp aka. Chevdor <chevdor@gmail.com>

    Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
    documentation files (the "Software"), to deal in the Software without restriction, including without limitation the
    rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit
    persons to whom the Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
    WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
    COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
    OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
