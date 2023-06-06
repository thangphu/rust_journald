// Test with "systemd-run --user --wait ./target/release/rust_proj"
// Other option is: cargo run ./target/release/rust_proj

use std::env;
use log::{error, warn, info, debug, set_max_level, LevelFilter};
use simple_logger::SimpleLogger;
use systemd_journal_logger::{connected_to_journal, init_with_extra_fields};

const MAX_ARGS: usize = 3;

fn main() {
    if connected_to_journal() {
        // If the output streams of this process are directly connected to the
        // systemd journal log directly to the journal to preserve structured
        // log entries (e.g. proper multiline messages, metadata fields, etc.)
        init_with_extra_fields(vec![("VERSION", env!("CARGO_PKG_VERSION"))]).unwrap();
    } else {
        // Otherwise fall back to logging to standard error.
        SimpleLogger::new().init().unwrap();
    }

    set_max_level(LevelFilter::Debug);

    let args: Vec<String> = env::args().collect();
    let argc = args.iter().count();

    debug!("***** Rust Starting Up *****");

    if argc > 1 {
        if argc <= MAX_ARGS {
            for (pos, arg) in args.iter().enumerate() {
                info!("arg {}: {}", pos, &arg);
            }
        } else {
            error!("Rust Error arguments exceeded {}", MAX_ARGS);
        }
    } else {
        warn!("Rust Warning. No arguments passed")
    }
}