extern crate hyper;
extern crate toml;

use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri;
use std::io::Read;
use std::fs::File;

fn main() {
    let config_path = std::path::Path::new("./web_conf.toml");
    let mut config_str = String::new();
    File::open(config_path).ok().expect("Failed to find config file web_conf.toml")
        .read_to_string(&mut config_str).unwrap();
    let config = toml::Parser::new(&config_str).parse().unwrap();
    let root = match config.get("root").unwrap() {
        &toml::Value::String(ref root) => {
            root.clone()
        },
        _ => { panic!("root should be a string") }
    };
    println!("Setting web root to {}", root);
    Server::http("0.0.0.0:4000").unwrap().handle(move |req: Request, res: Response| {
        let mut root = root.clone();
        match req.uri {
            RequestUri::AbsolutePath(path) => {
                root.push_str(&path[..]);
                match File::open(&root).ok() {
                    Some(ref mut file) => {
                        let mut res_str = String::new();
                        file.read_to_string(&mut res_str).unwrap();
                        res.send(res_str.as_bytes()).unwrap();
                    },
                    _ => {
                        res.send(b"404 Page not found").unwrap();
                    } // 404
                }
            }
            _ => {}
        }
    }).unwrap();
}
