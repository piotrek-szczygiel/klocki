<p align="center"><a href="https://github.com/piotrek-szczygiel/klocki"><img src="resources/logo.png" alt="Logo"></a></p>
<h1 align="center">Klocki</h1>
<p align="center">
    <img src="https://travis-ci.org/piotrek-szczygiel/klocki.svg?branch=master">
    <img src="https://ci.appveyor.com/api/projects/status/vjb1uy5nf7306jys/branch/master?svg=true">
    <img src="https://tokei.rs/b1/github/piotrek-szczygiel/klocki">
    <img src="https://img.shields.io/github/v/release/piotrek-szczygiel/klocki?include_prereleases&label=version">
    <img src="https://img.shields.io/github/downloads/piotrek-szczygiel/klocki/total">
    <img src="https://img.shields.io/github/release-date-pre/piotrek-szczygiel/klocki?label=last%20release">
    <img src="https://img.shields.io/github/last-commit/piotrek-szczygiel/klocki">
</p>
<hr>

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
cargo build
```

## Debian dependencies

```sh
sudo apt-get install libasound2-dev libudev-dev
```

## Arch dependencies

```sh
sudo pacman -S alsa-lib gcc-libs systemd-libs
```
