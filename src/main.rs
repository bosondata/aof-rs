extern crate regex;
#[macro_use]
extern crate clap;
extern crate resp;

mod filter;

use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::process;

use clap::{App, Arg};

use filter::{AOFParser, SimpleFilter};


fn main() {
    let matches = App::new("aof")
                      .version(env!("CARGO_PKG_VERSION"))
                      .author("Messense Lv <messense@icloud.com")
                      .about("Redis AppendOnly file filter")
                      .arg(Arg::with_name("dbs")
                             .help("Databases to show. Can be specified multiple times")
                             .short("d")
                             .long("database")
                             .multiple(true)
                             .takes_value(true))
                      .arg(Arg::with_name("aof_file")
                             .value_name("FILE")
                             .help("Path to the AOF file")
                             .required(true)
                             .index(1))
                      .get_matches();

    let path_arg = matches.value_of("aof_file").unwrap();
    let path = Path::new(path_arg);
    if !path.exists() {
        println!("AOF file {} does not exist", path_arg);
        process::exit(128);
    }
    let mut filter = SimpleFilter::new();
    if matches.is_present("dbs") {
        for db in values_t!(matches.values_of("dbs"), u32).unwrap_or_else(|e| e.exit()) {
            filter.add_database(db);
        }
    }
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    AOFParser::new(&mut reader, filter).filter();
}
