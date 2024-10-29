# pluggable-wasm-cli-demo

A demo of a formatter CLI with pluggable conversion logic by WebAssembly (Wasm). 

## Build

```
$ cd uppercase
$ cargo component build --target wasm32-unknown-unknown

$ cd lowercase
$ cargo component build --target wasm32-unknown-unknown
```

## Run

```
$ cargo run ../uppercase/target/wasm32-unknown-unknown/debug/uppercase.wasm "Hello World"
HELLO WORLD

$ cargo run ../lowercase/target/wasm32-unknown-unknown/debug/lowercase.wasm "Hello World"
hello world
```

## How I created this repo

### Create a project

```
$ mkdir pluggable-wasm-cli-demo
$ cd pluggable-wasm-cli-demo
```

### Define a formatter interface

```
$ mkdir wit
$ vim wit/world.wit
package tnantoka:formatter;

interface formattable {
    format: func(text: string) -> string;
}

world formattable-provider {
    export formattable;
}
```

### Create a uppercase formatter

```
$ cargo component new --lib uppercase
$ rm -rf uppercase/wit
$ cd uppercase

$ vim Cargo.toml
- package = "component:uppercase"
+ target = { path = "../wit", world = "formattable-provider" }

$ vim src/lib.rs
- use bindings::Guest;
+ use crate::bindings::exports::tnantoka::formatter::formattable::Guest;

-     /// Say hello!
-     fn hello_world() -> String {
-         "Hello, World!".to_string()
-     }
+     fn format(text: String) -> String {
+         text.to_uppercase()
+     }

$ cargo component build --target wasm32-unknown-unknown
$ cd ..
```

### Create a lowercase formatter

```
$ cargo component new --lib lowercase
...
```

### Create a CLI

```
$ cargo new cli
$ cd cli

$ cargo add anyhow
$ cargo add clap --features derive
$ cargo add wasmtime

$ vim src/main.rs
use clap::Parser;
use wasmtime::{Engine, Store};
use wasmtime::component::{bindgen, Component, Linker};

bindgen!({
    world: "formattable-provider",
    path: "../wit",
});

#[derive(Parser, Debug)]
struct Args {
    wasm_file: String,
    text: String,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = start(args) {
        eprintln!("Error: {}", e);
    }
}

fn start(args: Args) -> anyhow::Result<()> {
    let engine = Engine::default();

    let component = Component::from_file(&engine, &args.wasm_file)?;

    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    let provider = FormattableProvider::instantiate(&mut store, &component, &linker)?;

    let formattable = provider.tnantoka_formatter_formattable();

    let formatted = formattable.call_format(&mut store, &args.text)?;
    println!("{}", formatted);

    Ok(())
}

$ cargo build
    Finished
```

## Acknowledgements

- https://gihyo.jp/book/2024/978-4-297-14413-5
