# aof-rs - Redis appendonly file filter

[![Build Status](https://travis-ci.org/bosondata/aof-rs.svg?branch=master)](https://travis-ci.org/bosondata/aof-rs)

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
