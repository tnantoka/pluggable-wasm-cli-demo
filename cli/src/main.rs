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
