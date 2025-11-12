# Odyssey

# Dependencies

Odyssey relies on SuiteSparse for some matrix related operations
SuiteSparse is imported as a git submodule.

```
git submodule update --init
 ```

## Rust
Rust installation  see. [this page](https://rust-lang.org/tools/install/)
## Build

```
cargo b
```

## Usage
The generated binaires are located in 

```
./target/debug/odyssey [OPTIONS]
```

```
Usage: odyssey [OPTIONS] <COMMAND>

Commands:
  create    Create a project
  database  Manage database
  search    Search entry in imported databases
  run       Execute inventory, impact assessment and life cycle assessment
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Global verbosity flag
  -h, --help     Print help
  -V, --version  Print version
```

## Contribute


## FAQs
