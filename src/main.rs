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
    
    let loop_len = 6.0f32;
    let load_thresh: f32 = 1.0;
    
    for _i in 0..100 {
        let loop_start = Instant::now();
        let current_load = parse_load(&mut load_buf);
        
        if current_load[0] > load_thresh {
            let ts_now: DateTime<Utc> = Utc::now();
            println!("{} | {:?}", ts_now, current_load);
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
                for (ind, field) in v.split_whitespace().take(3).enumerate() {
                    //~ println!("{} {}", i, x);
                    match field.parse::<f32>() {
                        Ok(f) => out[ind] = f,
                        Err(_) => (),
                    }
                }
            },
            Err(_) => (),
        }
    }
    out
}
