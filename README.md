# Klocki

[![Build Status](https://travis-ci.org/piotrek-szczygiel/klocki.svg?branch=master)](https://travis-ci.org/piotrek-szczygiel/klocki)
[![Build Status](https://ci.appveyor.com/api/projects/status/vjb1uy5nf7306jys/branch/master?svg=true)](https://ci.appveyor.com/project/piotrek-szczygiel/klocki/branch/master)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/piotrek-szczygiel/klocki/blob/master/LICENSE)
[![Lines of Code](https://tokei.rs/b1/github/piotrek-szczygiel/klocki)](https://github.com/piotrek-szczygiel/klocki)

An arcade game written in [Rust](https://www.rust-lang.org) using the [ggez](https://github.com/ggez/ggez) library.

## Supported platforms

* Windows
* Linux
* Mac

## Download

Visit the [release](https://github.com/piotrek-szczygiel/klocki/releases) tab.

## Resources

Music created by [Patrick de Arteaga](https://patrickdearteaga.com).  
Icon made by [Vitaly Gorbachev](https://www.flaticon.com/authors/vitaly-gorbachev) from [www.flaticon.com](https://www.flaticon.com).  
Some assets taken from [NullpoMino](https://github.com/nullpomino/nullpomino) project.

## Development

Easiest way to obtain cargo is from [rustup.rs](https://rustup.rs).

```sh
git clone https://github.com/piotrek-szczygiel/klocki
cd klocki

cargo build             # debug
cargo build --release   # release

cargo run --release     # run release version
```

## Debian dependencies

```sh
sudo apt-get install libasound2-dev libudev-dev
```

## Arch dependencies

```sh
sudo pacman -S alsa-lib gcc-libs systemd-libs
```
