extern crate toml;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

fn test_str() {
    #[derive(Deserialize)]
    struct Config {
        ip: String,
        port: Option<u16>,
        keys: Keys,
    }

    #[derive(Deserialize)]
    struct Keys {
        github: String,
        travis: Option<String>,
    }

    let config: Config = toml::from_str(r#"
        ip = '127.0.0.1'

        [keys]
        github = 'xxxxxxxxxxxxxxxxx'
        travis = 'yyyyyyyyyyyyyyyyy'
    "#).unwrap();

    assert_eq!(config.ip, "127.0.0.1");
    assert_eq!(config.port, None);
    assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
    assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
}

fn test_file() {
    #[derive(Debug)]
    #[derive(Deserialize)]
    struct Conf {
        database:DB
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    struct DB {
        server: String,
        ports: Vec<u32>,
        connection_max: u32,
        enabled: bool,
        country: Option<String>
    }

    use std::fs::File;
    use std::io::prelude::*;

    fn file_to_string(f: &str) -> std::io::Result<String> {
        let mut file = File::open(f)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    let content = match file_to_string("examples/simple.toml") {
        Ok(v) => v,
        Err(e) => panic!("error:{}", e),
    };

    let database_conf: Conf = toml::from_str(&*content).unwrap();

    println!("database_conf={:?}", database_conf);
}

fn main() {
    test_file();
}