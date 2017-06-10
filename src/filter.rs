use std::str;
use std::io::{self, Read, Write, BufReader};
use regex::Regex;

use resp::{Value, Decoder};

pub trait Filter {
    fn matches_db(&self, _db: u32) -> bool {
        true
    }
    fn matches_key(&self, _key: &str) -> bool {
        true
    }
    fn matches_cmd(&self, _cmd: &str) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct SimpleFilter {
    databases: Vec<u32>,
    keys: Option<Regex>,
    commands: Vec<String>,
}

impl SimpleFilter {
    pub fn new() -> Self {
        SimpleFilter {
            databases: vec![],
            keys: None,
            commands: vec![],
        }
    }

    pub fn add_database(&mut self, db: u32) {
        self.databases.push(db);
    }

    pub fn add_keys(&mut self, key: Regex) {
        self.keys = Some(key);
    }

    pub fn add_command(&mut self, cmd: String) {
        self.commands.push(cmd);
    }
}

impl Filter for SimpleFilter {
    fn matches_db(&self, db: u32) -> bool {
        if self.databases.is_empty() {
            true
        } else {
            self.databases.iter().any(|&x| x == db)
        }
    }

    fn matches_key(&self, key: &str) -> bool {
        match self.keys.clone() {
            None => true,
            Some(re) => re.is_match(key),
        }
    }

    fn matches_cmd(&self, cmd: &str) -> bool {
        if self.commands.is_empty() {
            true
        } else {
            self.commands.iter().any(|x| x == cmd)
        }
    }
}

pub struct AOFParser<R: Read, F: Filter> {
    filter: F,
    decoder: Decoder<R>,
}

impl<R: Read, F: Filter> AOFParser<R, F> {
    pub fn new(input: BufReader<R>, filter: F) -> AOFParser<R, F> {
        AOFParser {
            filter: filter,
            decoder: Decoder::with_buf_bulk(input),
        }
    }

    pub fn filter(&mut self) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let mut current_db = None;
        while let Ok(value) = self.decoder.decode() {
            match value {
                Value::Array(ref vals) => {
                    let cmd;
                    let key;
                    if let Value::BufBulk(ref bytes) = vals[0] {
                        cmd = String::from_utf8_lossy(bytes);
                        if "SELECT" == &cmd {
                            if let Value::BufBulk(ref bytes) = vals[1] {
                                current_db = Some(String::from_utf8_lossy(bytes)
                                    .parse::<u32>()
                                    .unwrap());
                            }
                            key = None;
                        } else if let Value::BufBulk(ref bytes) = vals[1] {
                            key = Some(String::from_utf8_lossy(bytes))
                        } else {
                            key = None;
                        }
                        if let Some(db) = current_db {
                            if self.filter.matches_db(db) && self.filter.matches_cmd(&cmd) {
                                if let Some(key_str) = key {
                                    if self.filter.matches_key(&key_str) {
                                        handle.write(&value.encode()).unwrap();
                                    }
                                } else {
                                    handle.write(&value.encode()).unwrap();
                                }
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
