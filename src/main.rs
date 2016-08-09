extern crate regex;
#[macro_use]
extern crate clap;
extern crate resp;

mod filter;

use std::env;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::process;

use clap::App;

use filter::{AOFParser, SimpleFilter};


fn main() {
    let mut app = App::new("aof")
                      .version(env!("CARGO_PKG_VERSION"))
                      .author("Messense Lv <messense@icloud.com")
                      .about("Redis AppendOnly file filter")
                      .args_from_usage(
                        "-d, --database [dbs]...  'Databases to show. Can be specified multiple times'
                        <aof_file>              'Path to the AOF file'");

    let matches = app.get_matches_from_safe_borrow(env::args())
                     .unwrap_or_else(|_| {
                        process::exit(1)
                     });

    let path_arg = matches.value_of("aof_file").unwrap();
    let path = Path::new(path_arg);
    if !path.exists() {
        let _ = app.print_help();
        process::exit(128);
    }
    let mut filter = SimpleFilter::new();
    if matches.is_present("database") {
        for db in values_t!(matches.values_of("database"), u32).unwrap_or_else(|e| e.exit()) {
            filter.add_database(db);
        }
    }
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    AOFParser::new(&mut reader, filter).filter();
}
