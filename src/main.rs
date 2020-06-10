use std::collections::VecDeque;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
//~ use std::path::PathBuf;
use std::process::{self, Command};
use std::str;

extern crate chrono;

use std::thread;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};

use serde_derive::Deserialize;
#[derive(Deserialize)]
struct Config {
    load_threshold: f32,
    loop_length: f32,
    log_buffer_size: i16,
    commands: Vec<String>,
}

fn main() -> io::Result<()> {
    let mut cwd = env::current_exe().unwrap();
    cwd.pop();
    if cwd.file_name().unwrap() == "release" || cwd.file_name().unwrap() == "debug" {
        cwd.pop();
        cwd.pop();
    }
    cwd.push("config.toml");
    let config_raw = fs::read_to_string(cwd.to_str().unwrap())?;
    let config: Config = toml::from_str(&config_raw).unwrap_or_else(|err| {
        println!("error parsing config: {}", err);
        process::exit(1);
    });
    
    let load_f = File::open("/proc/loadavg")?;
    let mut load_buf = BufReader::new(load_f);
    
    let mem_f = File::open("/proc/meminfo")?;
    let mut mem_buf = BufReader::new(mem_f);
    
    //~ let loop_len = 6.0f32;
    //~ let load_thresh: f32 = 0.1;
    
    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("servermon.log")
        .unwrap();
        
    let mut log_writer = BufWriter::new(log_file);
    
    let mut log_buf: VecDeque<String> = VecDeque::with_capacity(config.log_buffer_size as usize);
    for _i in 0..config.log_buffer_size {
        log_buf.push_back(String::from(""));
    }
    
    for _i in 0..12 {
        let loop_start = Instant::now();
        let current_load = parse_load(&mut load_buf);
        let current_mem = parse_mem(&mut mem_buf);
        
        run_commands(&config.commands, &mut log_buf);
        
        if current_load[0] > config.load_threshold {
            let ts_now: DateTime<Utc> = Utc::now();
            println!("{} | {:?} | {:?}", ts_now, current_load, current_mem);
            
            for i in &log_buf {
                //~ println!("{}", i);
                log_writer.write_all(i.as_bytes())?;
                log_writer.flush()?;
                //~ log_buf.pop_front();
                //~ log_buf.push_back(String::from(""));
            }
            
            for _i in 0..config.log_buffer_size {
                log_buf.pop_front();
                log_buf.push_back(String::from(""));
            }
        }
        thread::sleep(Duration::from_secs_f32(config.loop_length)
            .checked_sub(loop_start.elapsed()).unwrap());
    }
    
    println!("----------end----------");
    //~ for i in log_buf {
        //~ println!("{}", i);
        //~ log_writer.write_all(i.as_bytes())?;
        //~ log_writer.flush()?;
        
        
    //~ }
    
    //~ for i in load_buf.lines()
    Ok(())
}

fn parse_load(buf: &mut BufReader<File>) -> [f32; 3] {
//~ fn parse_load(buf: &mut BufReader<File>) {
    buf.seek(io::SeekFrom::Start(0)).unwrap();
    //~ let mut out = [0.0f32; 3];
    let mut out = [0.0f32; 3];
    
    for line in buf.by_ref().lines() {
        match line {
            Ok(v) => {
                //~ println!("{}", v);
                for (field, arr) in v.split_whitespace().take(3).zip(out.iter_mut()) {
                    //~ println!("{} {}", i, x);
                    match field.parse::<f32>() {
                        Ok(f) => *arr = f,
                        Err(_) => (),
                    }
                }
            },
            Err(_) => (),
        }
    }
    out
}

fn parse_mem(buf: &mut BufReader<File>) -> [f32; 2] {
    buf.seek(io::SeekFrom::Start(0)).unwrap();
    let mut out = [0.0f32; 2];
    let mut ram_total = 0.0f32;
    let mut ram_avail = 0.0f32;
    let mut swap_total = 0.0f32;
    let mut swap_free = 0.0f32;
    
    for line in buf.by_ref().lines() {
        match line {
            Ok(v) => {
                let fields: Vec<_> = v.split_whitespace().collect();
                match fields[0] {
                    "MemTotal:" => {
                        match fields[1].parse::<f32>() {
                            Ok(f) => ram_total = f,
                            Err(_) => (),
                        }
                    },
                    "MemAvailable:" => {
                        match fields[1].parse::<f32>() {
                            Ok(f) => ram_avail = f,
                            Err(_) => (),
                        }
                    },
                    "SwapTotal:" => {
                        match fields[1].parse::<f32>() {
                            Ok(f) => swap_total = f,
                            Err(_) => (),
                        }
                    },
                    "SwapFree:" => {
                        match fields[1].parse::<f32>() {
                            Ok(f) => swap_free = f,
                            Err(_) => (),
                        }
                    },
                    _ => (),
                }
            },
            Err(_) => (),
        }
    }
    out[0] = (ram_total - ram_avail) / ram_total;
    out[1] = (swap_total - swap_free) / swap_total;
    out
}

fn run_commands(cmds: &Vec<String>, buf: &mut VecDeque<String>) {
    let mut out = String::new();
    for i in cmds {
        //~ println!("{}", i);
        let args: Vec<_> = i.split_whitespace().collect();
        let output = Command::new(args.iter().nth(0).unwrap())
            .args(args.iter().skip(1))
            .output()
            .expect("failed to execute");
            
        match str::from_utf8(&output.stdout) {
            Ok(v) => {
                //~ buf.pop_front();
                //~ buf.push_back(String::from(v));
                out.push_str(v);
            },
            Err(_) => (),
        }
    }
    out.push('\n');
    buf.pop_front();
    buf.push_back(out);
}
