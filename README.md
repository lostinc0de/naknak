# naknak
Implementation of the fun cipher Nak-Nak in Rust

## Nak-Nak cipher in Rust
This small project is a implementation of the fun substitution cipher naknak (duck-speak) in Rust.
It provides encoding and decoding functionality either as a library or a CLI program.
For more information see https://www.dcode.fr/nak-nak-duckspeak.
To check out the CLI, clone the repo and run

```bash
cargo r -- -e STRING_TO_ENCODE
cargo r -- -d STRING_TO_DECODE
```

For example

```bash
cargo r -- -e "Hello world!"
```

should return

```bash
Nak? Nak. Naknak nak? Naknak naknak Naknak naknak Naknak naknaknak Nananak Nak Naknaknak Naknaknak Naknak naknaknak Naknaknak Nananak Naknak naknak Naknak Nak? Nananak Nanak
```
