# Game of Life

[![Build Status][travis-badge]][travis-project] [![codecov][codecov-badge]][codecov-project]

Little project which implements [Conway's Game of Life][wiki-gol].

The API doc of the whole crate is [here][crate-doc].

## Build and Run

To buid the game you need [Rust][rust-lang] and [Cargo][cargo-tool] installed. Then type:

```bash
cargo clean
cargo test
cargo build --release
```

Then you can invoke the game with:

```bash
./target/release/game_of_life -h
```

To see the available options or without `-h` to run it with defaults.

## The Rules

There are four simple rules when a living cell dies or a new cell will be born:

- `C` living cell
- `N` neighbour cell
- `x` died cell

### 1. Rule

A cell dies if it has **< 2 neighbours**:

```text
C -> x
```

or

```text
CN -> xN
```

### 2. Rule

A cell survives if it has **exactly 2 or 3 neighbours**:

```text
CN -> CN
N     N
```

or

```text
NCN -> NCN
 N      N
```

### 3. Rule

A cell dies if it has **> 3 neighbours**:

```text
 N      N
NCN -> NxN
 N      N
```

or

```text
NNN    NNN
NCN -> NxN
NNN    NNN
```

### 4. Rule

A new cell is born at an **empty place**, if this place has **exactly 3 neighbours**:

```text
C   -> CC
 CC     CC
```

[travis-project]:   https://travis-ci.org/Weltraumschaf/game_of_life
[travis-badge]:     https://travis-ci.org/Weltraumschaf/game_of_life.svg?branch=master
[codecov-project]:  https://codecov.io/gh/Weltraumschaf/game_of_life
[codecov-badge]:    https://codecov.io/gh/Weltraumschaf/game_of_life/branch/master/graph/badge.svg
[wiki-gol]:         https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
[crate-doc]:        https://weltraumschaf.github.io/game_of_life/game_of_life/index.html
[rust-lang]:        http://rust-lang.org/
[cargo-tool]:       https://doc.rust-lang.org/cargo/
