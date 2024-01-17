use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};

pub mod bindings {
    use wasmtime::component::bindgen;
    bindgen!(in "../../wit");
}
pub use bindings::exports::typst_community::unicode_math_class::crate_ as wit;

pub const WORLD: &str = "unicode-math-class";

pub struct MyState {}

pub fn instantiate() -> (bindings::UnicodeMathClass, Store<MyState>) {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    let component = Component::from_file(
        &engine,
        format!("../../target/wasm32-unknown-unknown/debug/{WORLD}.wasm"),
    )
    .unwrap();
    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, MyState {});
    let (bindings, _) =
        bindings::UnicodeMathClass::instantiate(&mut store, &component, &linker).unwrap();
    (bindings, store)
}
