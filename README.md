# Tetris (work in progress)

[![Build Status](https://travis-ci.org/piotrek-szczygiel/tetris.svg?branch=master)](https://travis-ci.org/piotrek-szczygiel/tetris)
[![Build status](https://ci.appveyor.com/api/projects/status/f84px445py8ldj24/branch/master?svg=true)](https://ci.appveyor.com/project/piotrek-szczygiel/tetris/branch/master)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/piotrek-szczygiel/tetris/blob/master/LICENSE)

Multiplayer Tetris clone written in [Rust](https://www.rust-lang.org) using the [ggez](https://github.com/ggez/ggez) library.

## Supported platforms

* Fully supported: Windows, Linux
* Might work: Mac

## Running

```sh
git clone https://github.com/piotrek-szczygiel/tetris && cd tetris
cargo run --release
```

## Linux dependencies

```sh
sudo apt-get install libasound2-dev libudev-dev
```
