use once_cell::sync::Lazy;
use oso::{Oso, PolarClass, ToPolar};
use rustler::{Error, NifStruct, NifUnitEnum, NifUntaggedEnum};
use std::sync::Mutex;

static OSO: Lazy<Mutex<Oso>> = Lazy::new(|| {
    let mut oso = Oso::new();

    //oso.register_class(Creds::get_polar_class()).unwrap();
    //oso.register_class(Survey::get_polar_class()).unwrap();
    //oso.load_file("policy.polar").unwrap();

    Mutex::new(oso)
});

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

rustler::init!("Elixir.Oso.Native", [add]);
