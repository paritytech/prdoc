# PRDoc

<figure>
<img src="https://github.com/paritytech/prdoc/actions/workflows/quick-check.yml/badge.svg?branch=master" alt="badge" />
</figure>

`prdoc` is a tool designed to help generating, checking and loading `.prdoc` files.
`.prdoc` files are YAML files adhering to defined JSON schema and helping with
code change documentation.

While platform like Github allow a simple description for a Pull Request (PR), this is
limited to a title, description and some labels.

The description of a PR itself is often used to describe the change but not
document it in a structured fashion.
A sample schema can be found here: [prdoc\_schema\_user.json](prdoc_schema_user.json) but each repository is free to
define its own JSON Schema.

## Features

-   [generate command](#_install) to create new PRDoc files

-   [scan command](#_scan): to quickly scan for PRDOc files in a folder

-   [check command](#_check): to check one or more PRDOc files

-   [load command](#_load): to load one or more PRDoc files

## Install

    cargo install prdoc

Alternatively, you may use a the container image if you prefer not installing anything on your system. See the
[Containers](#_containers) section for more details on containers.

## Philosophy

### Configuration, cli flags and environment variables

In order to provide a simple and uniform behavior in a repo, `prdoc` will search for a local configuration file.
The configuration file is a YAML file named `.prdoc.toml` or `prdoc.toml` and located in the root of the repo.

The configuration file can alternatively be passed via ENV (`PRDOC_CONFIG`) or cli flag (`-c`|`--config`).
ENV and cli flags have precedence over the local configuration file.

### Simple to use

While most commands supports options, they are designed to be simple to use and require a minimal amount of user input
when either a config or an `.env` file is present.

## Authoring a PRDoc

### Without tooling

No tooling but a text editor is required to author a new PRDoc. You may simply copy the template from your repo.
The template is defined in the [???](#config):

    grep template *prdoc.toml

You then need to save the file as `pr_NNNN.prdoc` (where `NNN` is the PR number) in the repo’s prdoc folder.
This folder is also defined in the config (`./prdoc` is the default\`):

    grep output *prdoc.toml

### Using the `prdoc` cli

You will however find it more convenient to [install](https://github.com/paritytech/prdoc#install) and use the `prddoc`
cli and just run:

    prdoc generate 9999

After editing the PRDoc file, you may check whether is adheres to the schema using:

    prdoc check -n 1226

### Using VSCode

See the [Schemas](#schemas) chapter to learn how to configure VSCode to recognize and check PRDoc files.

### YAML Anchors

You may use YAML anchors as demonstrated below.

    # Schema: Parity PR Documentation Schema (prdoc)

    title: Foobar

    doc:
      - audience: Runtime User
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

## Config

Using a configuration file makes it easier for all users as they will be able to omit some of the required flags when
using the `prdoc`.

### Config file name and location

The config will be found if located at the root of the repo and named either:
- `prdoc.toml`
- `.prdoc.toml`

Alternatively, it can be defined as an ENV named `PRDOC_CONFIG` and contain the path of the config, relative to the
repository’s root.

### Content

    version = 1
    schema = "tests/data/sample_schema.json"
    output_dir = "/tmp/prdoc"
    prdoc_folders = ["tests/data/all", "tests/data/some"]
    template = "template.prdoc"

## Paths

In order to make it easier to use, `prdoc` and its configuration always refer to the **root of the repository**.

It means you can pass either absolute paths or relative ones but relatives ones are based on the root of the repo and
**not** the current working directory.

This allows users to use commands such as:

    prdoc check -n 1234
    # instead of:
    # prdoc check -n 1234 -d ../../folder/where/prdoc_files/are/stored

Or also:

    prdoc generate 1234
    # instead of;
    # prdoc generate 1234 -o ../../folder/where/prdoc_files/are/stored

## Schemas

### PR Doc

The documentation for PRs comes as a file with the extension `.prdoc`.
This is essentially a `yaml` file and the extension helps using the right JSON schema to validate the file.

In VScode, open your user settings and ensure you have the following section:

You first need to tell VScode that .prdoc files are YAML files:

    "files.associations": {
        "*.prdoc": "yaml",
    },

You then need to point to the right schemas:

     "yaml.schemas": {
        [...other schemas...]
        "/path/to/polkadot-sdk/prdoc/schema_user.json": "*polkadot-sdk*/**/*.prdoc",
        "/path/to/subxt/prdoc/schema_user.json": "*subxt*/**/*.prdoc"
      },

You need to restart/reload VSCode after those changes for the new settings to be picked up.

Should you initially have created the file with another extension such as `.txt`, make sure to change the format to
`YAML` in the VSCode status bar and the right schema should then be picked up.

## Usage

    prdoc is a utility to generate, check and load PRDoc files.

    More at <https://github.com/paritytech/prdoc>

    Usage: prdoc [OPTIONS] [COMMAND]

    Commands:
      generate  Generate a new file. It will be saved by default unless you provide --dry-run. The command will fail if the target file already exists
      check     Check one or more prdoc files for validity
      scan      Scan a directory for prdoc files based on their name
      load      Load one or more prdoc
      help      Print this message or the help of the given subcommand(s)

    Options:
      -c, --config <CONFIG>
              [env: PRDOC_CONFIG=]

      -d, --prdoc-folders <PRDOC_FOLDERS>
              [env: PRDOC_FOLDERS=]

      -v, --version
              Show the version

      -j, --json
              Output as json

      -h, --help
              Print help (see a summary with '-h')

### generate

    Generate a new file. It will be saved by default unless you provide --dry-run. The command will fail if the target file already exists

    Usage: prdoc generate [OPTIONS] <NUMBER>

    Arguments:
      <NUMBER>  Change number

    Options:
          --dry-run                        Do not save the generated document to file with the proper naming, show the content instead
      -c, --config <CONFIG>                [env: PRDOC_CONFIG=]
      -o, --output-dir <OUTPUT_DIR>        Optional output directory. It not passed, the default `PRDOC_DIR` will be used under the root of the current project
      -d, --prdoc-folders <PRDOC_FOLDERS>  [env: PRDOC_FOLDERS=]
      -j, --json                           Output as json
      -h, --help                           Print help

### check

    Check one or more prdoc files for validity

    Usage: prdoc check [OPTIONS]

    Options:
      -f, --file <FILE>                    Directly specify the file to be checked. It can be relative to the base directory
      -c, --config <CONFIG>                [env: PRDOC_CONFIG=]
      -n, --number <NUMBER>                number
      -d, --prdoc-folders <PRDOC_FOLDERS>  [env: PRDOC_FOLDERS=]
      -l, --list <LIST>                    Get the list of PR numbers from a file
      -s, --schema <SCHEMA>                Schema to be used. Passing this flag/ENV overrides the value from the config [env: PRDOC_SCHEMA=]
      -j, --json                           Output as json
      -h, --help                           Print help

### scan

    Scan a directory for prdoc files based on their name

    Usage: prdoc scan [OPTIONS]

    Options:
      -a, --all                            Also return invalid files
      -c, --config <CONFIG>                [env: PRDOC_CONFIG=]
      -s, --sort                           Sort the output
      -d, --prdoc-folders <PRDOC_FOLDERS>  [env: PRDOC_FOLDERS=]
      -j, --json                           Output as json
      -h, --help                           Print help

### load

    Load one or more prdoc

    Usage: prdoc load [OPTIONS]

    Options:
      -f, --file <FILE>                    file path
      -c, --config <CONFIG>                [env: PRDOC_CONFIG=]
      -n, --number <NUMBER>                One or more PR numbers. Depending on the host OS, the max length of a command may differ. If you run into issues, make sure to check the `--list` option instead
      -d, --prdoc-folders <PRDOC_FOLDERS>  [env: PRDOC_FOLDERS=]
      -l, --list <LIST>                    Get the list of PR numbers from a file
      -j, --json                           Output as json
      -h, --help                           Print help

## Containers

If you prefer not having to install Rust & Cargo and have Podman or Docker installed, you may prefer to run a containerized
version of `prdoc`. This chapter explains how to proceed.

prdoc is designed to work at the repository level and you need to mount your repo as `/repo` into the prdoc container.

    podman run --rm -it -v $PWD:/repo paritytech/prdoc --help

        ENGINE=podman
        DOC_PATH="$PWD/tests/data/some"
        $ENGINE run --rm -it -v $DOC_PATH:/repo paritytech/prdoc --help
        $ENGINE run --rm -it -v $DOC_PATH:/repo paritytech/prdoc scan --all
        $ENGINE run --rm -it -v $DOC_PATH:/repo paritytech/prdoc check
        $ENGINE run --rm -it -v $DOC_PATH:/repo paritytech/prdoc load

The container image is working by default in `/repo` so it makes it simpler if you mount your repo there as shown
above.

### Run

    podman run --rm -it -v $PWD:/repo paritytech/prdoc --help

        ENGINE=podman
        DOC_PATH="$PWD/tests/data/some"
        $ENGINE run --rm -it -v $DOC_PATH:/repo paritytech/prdoc --help
        $ENGINE run --rm -it -v $DOC_PATH:/repo paritytech/prdoc scan --all
        $ENGINE run --rm -it -v $DOC_PATH:/repo paritytech/prdoc check
        $ENGINE run --rm -it -v $DOC_PATH:/repo paritytech/prdoc load

The container image is working by default in `/repo` so it makes it simpler if you mount your repo there as shown
above.

Commands can end up quite lengthy so you may like to set an alias:

        alias prdoc='podman run --rm -it -v $PWD:/repo paritytech/prdoc'

After setting this alias, you may use `prdoc` by simply invoking the `prdoc` command:

        prdoc --version

This is out of the scope of this documentation but note that you can just invoke `prdoc check` and expect it to work in
your repo as long as it contains a valid configuration file and schema. Check out the [???](#Configuration) chapter for more
details.

### Build

You can pull the container image from `paritytech`/`prdoc` or build you own:

        podman build -t prdoc .

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
