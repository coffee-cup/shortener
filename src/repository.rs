use crate::shortener::Shortener;
use std::{collections::HashMap, env, ptr::hash};

use redis::Commands;

pub trait Cache {
    // Store an entry and return an ID
    fn store(&mut self, data: &str) -> String;

    // Look up a previously stored entry
    fn lookup(&mut self, id: &str) -> Option<String>;
}

pub struct MemoryRepository {
    urls: HashMap<String, String>,
    shortener: Shortener,
}

impl MemoryRepository {
    pub fn new() -> MemoryRepository {
        MemoryRepository {
            urls: HashMap::new(),
            shortener: Shortener::new(),
        }
    }
}

impl Cache for MemoryRepository {
    fn store(&mut self, url: &str) -> String {
        let id = self.shortener.next_id();
        self.urls.insert(id.to_string(), url.to_string());
        id
    }

    fn lookup(&mut self, id: &str) -> Option<String> {
        self.urls.get(id).map(|s| s.clone())
    }
}

pub struct RedisRepository {
    con: redis::Connection,
    generator: harsh::Harsh,
}

impl RedisRepository {
    pub fn new() -> Result<RedisRepository, redis::RedisError> {
        let conn_string = match env::var("REDIS_URL") {
            Ok(val) => val,
            Err(_e) => panic!("REDIS_URL is required"),
        };

        println!("RedisRepository::new");

        let client = redis::Client::open(conn_string)?;
        let con = client.get_connection()?;

        let generator = harsh::Harsh::default();

        Ok(RedisRepository { con, generator })
    }
}

impl Cache for RedisRepository {
    fn store(&mut self, url: &str) -> String {
        println!("HELLO");

        let keys: Vec<String> = self.con.keys("*").unwrap();
        let count = keys.len() as u64;
        let hashed = self.generator.encode(&[count]);

        let _: () = self.con.set(hashed.clone(), url).unwrap();

        hashed
    }

    fn lookup(&mut self, id: &str) -> Option<String> {
        let url: Option<String> = match self.con.get(id) {
            Ok(val) => Some(val),
            Err(_e) => None,
        };

        url
    }
}
