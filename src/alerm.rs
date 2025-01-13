use chrono::{Local, Timelike};

use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use std::fs;
use crate::components;

fn now_hour() -> u32 {
    Local::now().hour()
}

fn now_minute() -> u32 {
    Local::now().minute()
}

fn now_second() -> u32 {
    Local::now().second()
}

fn alerts_check(joined_time: &str) -> Result<bool, Box<dyn std::error::Error>>{
    let mut file = fs::File::open("./.puyu/alerts")?;

    let mut file_buffer = String::new();

    file.read_to_string(&mut file_buffer)?;

    let time_: String = file_buffer.chars().skip(10).collect();

    Ok(time_ == joined_time)
}

pub fn run_pending() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let hour = now_hour();
        let minute = now_minute();
        let second = now_second();

        let joined = format!("{}:{}:{}", hour, minute, second);

        if alerts_check(&joined)? {
            components::footer_draw("Alerm!!!")?;
            for _ in 0..50 {
                print!("\x07");
                sleep(Duration::from_millis(100));
            }
        }

        sleep(Duration::from_millis(200));
    }
}