extern crate env_logger;
extern crate libgb;
#[macro_use]
extern crate log;

use std::env;

use libgb::Gameboy;

fn main() {
    let mut builder = env_logger::Builder::new();
    if let Ok(config) = env::var("GB_LOG") {
        builder.parse(&config);
    }
    builder.init();

    let mut gb = Gameboy::new();
    if let Some(path) = std::env::args().nth(1) {
        match gb.load(path) {
            Err(err) => error!("Could not load ROM: {:?}", err),
            _ => (),
        }
    }

    gb.run()
}
