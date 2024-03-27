use redis::{self, Client, Connection, ConnectionLike, RedisError};
use std::{ self, io::{stdin, stdout, Read, Write}};
use structopt::StructOpt;
use options::Options;
use regex::Regex;
pub mod options;
fn main() {
    let opts = Options::from_args();
    let uri = opts.uri.unwrap_or(String::from("redis://127.0.0.1/"));    
    let result : Result<(), String>;
    if let Some(channel) = opts.subscribe {
        result = match get_connection(uri).map_err(redis_err) {
            Ok(cn) => {
                start_subscription_listener(cn,channel)
            },
            Err(e) => Err(e),
        };
    }else{
        result = match get_connection(uri).map_err(redis_err) {
            Ok(cn) => {
                match opts.live {
                    true => run_commands_linebyline(cn),
                    false => run_commands_til_eof(cn),
                }
            },
            Err(e) => Err(e),
        }
    }

    match result {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            print!("{}", e);
            std::process::exit(-2)
        }
    };
}

fn start_subscription_listener(mut con: Connection, channel: String) -> Result<(), String>{
    let mut pubsub = con.as_pubsub();
    let _sub = pubsub.subscribe(channel)
        .map_err(redis_err);
    loop{
        let msg = pubsub.get_message().map_err(redis_err)?;   
        stdout().write_all(msg.get_payload_bytes()).map_err(|e| e.to_string())?;
        println!();
        stdout().flush().unwrap();
    }
}

fn run_commands_linebyline(mut con: Connection) -> Result<(), String> {
    for read_attempt in stdin().lines(){
        let line = read_attempt.map_err(|e| e.to_string())?;
        if line == "quit" { std::process::exit(0); }
        run_single_command(line.as_str(), &mut con)?;
    }
    Ok(())
}

fn get_connection(uri: String) -> Result<Connection, RedisError>{
    let client = Client::open(uri)?;
    client.get_connection()
}

fn run_commands_til_eof(mut con : Connection) -> Result<(), String> {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .map_err(|e| e.to_string())?;

    for line in buffer.lines() {
        run_single_command(line, &mut con)?;
    }
    Ok(())
}

fn run_single_command(line: &str, con: &mut redis::Connection) -> Result<(), String> {
        let cmd = build_command(line)?;
        let rval = con.req_command(&cmd).map_err(redis_err)?;
        write_output(&rval)?;
    Ok(())
}

fn write_output(rval: &redis::Value) -> Result<(), String> {
    match rval {
        redis::Value::Nil => { /* left empty */ }
        redis::Value::Int(v) => println!("{}", v),
        redis::Value::Data(v) => {
            stdout().write_all(v).map_err(|e| e.to_string())?;
            println!();
        }
        redis::Value::Bulk(vals) => {
            for v in vals {
                write_output(v)?;
            }
        }
        redis::Value::Status(v) => println!("{}", v),
        redis::Value::Okay => println!("OK"),
    };
    Ok(())
}

fn redis_err(e: RedisError) -> String {
    e.to_string()
}

fn build_command(command: &str) -> Result<redis::Cmd, String> {
    let r = Regex::new(r#""(?:([^"]*))"|\S+"#).unwrap();
    let mut parts = r.find_iter(command).map(|x| x.as_str().trim_matches('"'));
    match parts.nth(0) {
        Some(c) => {
            let mut rcomm = redis::cmd(c);
            for arg in parts {
                rcomm.arg(arg);
            }
            Ok(rcomm)
        }
        None => Err(String::from("no valid command was provided")),
    }
}
