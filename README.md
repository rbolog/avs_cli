# avs_cli
A Swiss OASI AHV AVS 13 generator and validator.

Command line app. to generate and validate a swiss social number.
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

### cross compile

[see](https://github.com/cross-rs/cross) 

`cross build --release --target x86_64-pc-windows-gnu`

`cross build --release --target aarch64-unknown-linux-gnu`

## Howto

```shell
avs_cli --help
```

```text
A Swiss AVS 13 generator and validator.

Usage: avs_cli [OPTIONS] [NAVS13]

Arguments:
  [NAVS13]
          NAVS13 to validate.
          Note that only the structure is validated. This is not enough to make it effective.
          Example: 756.1234.5678.97

Options:
  -c, --create
          Creates a structurally valid navs13 for test purposes.
  -n, --number <NUMBER>
          Number of NAVS13 to generate. max=255 [default: 1]
  -h, --help
          Print help information
  -V, --version
          Print version information
```


```shell
avs_cli --create --number 10
```

```text
756.7604.1935.05
756.1288.4168.44
756.8671.3496.11
756.0437.4257.20
756.2614.0504.04
756.4469.2738.85
756.1718.3383.02
756.7198.9173.58
756.4133.2140.84
756.7013.0557.91
```
---
```shell
avs_cli 756.7604.1935.05
```

or

```shell
avs_cli 7567604193505
```

```text
756.7604.1935.05 is valid.
```

---

```shell
avs_cli 756.7604.1934.05
```

```text
756.7604.1934.05 is invalid. Error code 66, description 5 Is invalid EAN-13.
```
