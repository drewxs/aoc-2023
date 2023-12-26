# AoC 2023

My solutions in Rust.

## Setup

Instead of creating input files, we're auto-fetching and caching the input data using a bare-bones client in `src/aoc.rs`.

This data is stored in the `cache` dir, and used from there for each solution.

You can manually override the input data by altering the cache files, they will only refetch if the file doesn't exist.

Retrieve session cookie by logging to [AoC](https://adventofcode.com), then inspecting the cookie request header for the page request (session=`***`).

Create `.env` with `AOC_TOKEN=your_session_cookie`.

## Usage

```sh
cargo run # all
cargo run <day> <part>
```

Clear cache:

```sh
rm -rf cache
```
