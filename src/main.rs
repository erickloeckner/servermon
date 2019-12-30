use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
//~ use std::path::PathBuf;
extern crate chrono;

use std::thread;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};

fn main() -> io::Result<()> {
    let load_f = File::open("/proc/loadavg")?;
    let mut load_buf = BufReader::new(load_f);
    
    let mem_f = File::open("/proc/meminfo")?;
    let mut mem_buf = BufReader::new(mem_f);
    
    let loop_len = 6.0f32;
    let load_thresh: f32 = 0.1;
    
    for _i in 0..20 {
        let loop_start = Instant::now();
        let current_load = parse_load(&mut load_buf);
        let current_mem = parse_mem(&mut mem_buf);
        
        if current_load[0] > load_thresh {
            let ts_now: DateTime<Utc> = Utc::now();
            println!("{} | {:?} | {:?}", ts_now, current_load, current_mem);
        }
        thread::sleep(Duration::from_secs_f32(loop_len)
            .checked_sub(loop_start.elapsed()).unwrap());
    }
    
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
