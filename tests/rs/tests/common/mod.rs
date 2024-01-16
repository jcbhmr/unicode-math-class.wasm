use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Config, Engine, Store};

bindgen!(in "../../wit");

pub struct MyState {}

pub fn instantiate() -> (UnicodeMathClassWorld, Store<MyState>) {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    let component = Component::from_file(
        &engine,
        "../../target/wasm32-unknown-unknown/debug/unicode_math_class_wasm.wasm",
    )
    .unwrap();
    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, MyState {});
    let (bindings, _) =
        UnicodeMathClassWorld::instantiate(&mut store, &component, &linker).unwrap();
    (bindings, store)
}
