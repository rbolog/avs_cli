# avs_cli
A Swiss AVS 13 generator and validator.

Command line app. to generate and validate a swiss navs13.
This utility is for testing purposes, only the structure is validated which does not mean
that the number is administratively valid.
[navs13](https://www.zas.admin.ch/zas/fr/home/partenaires-et-institutions-/navs13.html)
Algorithm is described in annexe 8 :
[Directives](https://sozialversicherungen.admin.ch/fr/d/6938#)

## build

### prerequisite
* [Rust](https://www.rust-lang.org/tools/install)

### create release (all platform)
```sh
git clone https://github.com/rbolog/avs_cli.git;
cd avs_cli
cargo build release
```

for binaries see release

## Howto

`avs_cli --help`
