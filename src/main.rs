use wasmer::wat2wasm;
use wasmer::imports;
use wasmer::{Instance, Module, Store};
use wasmer::{TypedFunction};

macro_rules! from_impl {
    ($source:path, $target:path, $variant:ident) => {
        impl From<$source> for $target {
            fn from(val: $source) -> Self { Self::$variant(val) }
        }
    }
}

macro_rules! error_enum {
    ($id:ident { $( $variant:ident ( $arg: ty )),* $(,)? }) => {
        #[derive(Debug)]
        enum $id { $( $variant($arg) ),* }
        $( from_impl! { $arg, $id, $variant } )*
    }
}

error_enum! {
    WasmerError {
        Wat(wat::Error),
        Compile(wasmer::CompileError),
        Instantiation(wasmer::InstantiationError),
        Export(wasmer::ExportError),
        Runtime(wasmer::RuntimeError)
    }
}

fn main() -> Result<(), WasmerError> {
    let wasm_bytes = wat2wasm(br#"
(module
  (type $add_one_t (func (param i32) (result i32)))
  (func $add_one_f (type $add_one_t) (param $value i32) (result i32)
    local.get $value
    i32.const 1
    i32.add)
  (export "add_one" (func $add_one_f)))
"#)?;

    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let imports = imports! {};
    let instance = Instance::new(&mut store, &module, &imports)?;

    let add_one = instance.exports.get_function("add_one")?;
    let add_one: TypedFunction<i32, i32> = add_one.typed(&store)?;

    let result: i32 = add_one.call(&mut store, 3)?;
    println!("Hello, world!: {}", result);

    Ok(())
}
