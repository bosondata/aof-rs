# aof-rs - Redis appendonly file filter

[![Build Status](https://travis-ci.org/bosondata/aof-rs.svg?branch=master)](https://travis-ci.org/bosondata/aof-rs)
[![Crates.io](https://img.shields.io/crates/v/aof.svg)](https://crates.io/crates/aof)

## Installation

You can install ``aof-rs`` using Cargo:

```bash
$ cargo install aof
```

## Build

```bash
$ cargo build --release
```

To build a static linked binary, use the ``x86_64-unknown-linux-musl`` target:

```bash
$ cargo build --release --target=x86_64-unknown-linux-musl
```

## Usage

### Filter by database

```bash
$ aof /usr/lib/redis/appendonly.aof -d 1
```

### Filter by command

```bash
$ aof /usr/lib/redis/appendonly.aof -c hmset
```

### Filter by key pattern

```bash
$ aof /usr/lib/redis/appendonly.aof -k '^*:count$'
```

You can combine all these filters together to have better control.

### Restore to Redis

First, filter out the stuffs you want and save it to some file:

```bash
$ aof /usr/lib/redis/appendonly.aof -d 1 > ~/db1.aof
```

Then pipe the filtered AOF file to ``redis-cli`` to mass insert data to another Redis:

```bash
$ cat ~/db1.aof | redis-cli -n 1 --pipe
```

## License

The MIT License (MIT)

Copyright (c) 2016 BosonData

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
