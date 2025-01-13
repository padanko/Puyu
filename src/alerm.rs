use chrono::{Local, Timelike};
use rodio::{Decoder, OutputStream};

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

    let time: String = file_buffer.chars().skip(10).collect();

    Ok(time == joined_time)
}

pub fn run_pending() -> Result<(), Box<dyn std::error::Error>> {

    loop {
        let hour = now_hour();
        let minute = now_minute();
        let second = now_second();

        let joined = format!("{}:{}:{}", hour, minute, second);

        if alerts_check(&joined)? {
            components::footer_draw("Alerm")?;
            let file = fs::File::open("./.puyu/alerm.wav")?;
            let (_stream, handle) = OutputStream::try_default()?;
            let sink = rodio::Sink::try_new(&handle)?;
            let source = Decoder::new(std::io::BufReader::new(file))?;
            sink.append(source);
            sink.sleep_until_end();
        }

        sleep(Duration::from_millis(10));
    }
}