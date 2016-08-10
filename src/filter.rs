use std::str;
use std::io::BufRead;
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

pub struct AOFParser<R: BufRead, F: Filter> {
    input: R,
    filter: F,
    decoder: Decoder,
}

impl<R: BufRead, F: Filter> AOFParser<R, F> {
    pub fn new(input: R, filter: F) -> AOFParser<R, F> {
        AOFParser {
            input: input,
            filter: filter,
            decoder: Decoder::with_buf_bulk(),
        }
    }

    pub fn filter(&mut self) {
        let mut buffer = String::new();
        let mut current_db = None;
        while self.input.read_line(&mut buffer).unwrap() > 0 {
            self.decoder.feed(buffer.as_bytes()).unwrap();
            buffer.clear();
            if let Some(value) = self.decoder.read() {
                match value {
                    Value::Array(ref vals) => {
                        let cmd;
                        let key;
                        if let Value::BufBulk(bytes) = vals[0].clone() {
                            cmd = String::from_utf8(bytes).unwrap();
                            if "SELECT" == &cmd {
                                if let Value::BufBulk(bytes) = vals[1].clone() {
                                    current_db = Some(String::from_utf8(bytes)
                                        .unwrap()
                                        .parse::<u32>()
                                        .unwrap());
                                }
                                key = None;
                            } else {
                                if let Value::BufBulk(bytes) = vals[1].clone() {
                                    key = Some(String::from_utf8(bytes).unwrap())
                                } else {
                                    key = None;
                                }
                            }
                            if let Some(db) = current_db {
                                if self.filter.matches_db(db) {
                                    if self.filter.matches_cmd(&cmd) {
                                        if let Some(key_str) = key {
                                            if self.filter.matches_key(&key_str) {
                                                print!("{}", value.to_encoded_string().unwrap());
                                            }
                                        } else {
                                            print!("{}", value.to_encoded_string().unwrap());
                                        }
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
}
