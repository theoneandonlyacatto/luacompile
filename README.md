# luacompile

Fast Lua compiler with LuaJIT support, written in Rust.

## Installation

`pip install luacompile`

## Usage
```python
import luacompile
Compile Lua source code
compiled = luacompile.compile(
source="function add(a, b) return a + b end",
strip=True,
name="test.lua"
)
Write to file
with open("output.luac", "wb") as f:
f.write(compiled)```


## Features

- Fast compilation using LuaJIT
- Cross-platform support (Windows, Linux, MacOS)
- Python 3.8+ support
- Optional debug info stripping