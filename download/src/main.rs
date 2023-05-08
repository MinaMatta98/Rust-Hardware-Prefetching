use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;
use std::env;
pub mod buttonhandler;

struct RxBytes {
    rx: i64,
}

impl RxBytes {
    fn new() -> Self {
        let file = File::open("/sys/class/net/eno1/statistics/rx_bytes").unwrap();
        let mut down = String::new();
        BufReader::new(file).read_line(&mut down).unwrap();
        Self {
            rx: down.trim().parse::<i64>().unwrap(),
        }
    }

    fn delta(down_initial: Self, down_final: Self) -> i64 {
        (down_final.rx - down_initial.rx) / 1024
    }

    fn status_text(delta: i64) -> String {
        let mut p = "kB/s";

        match delta {
            999.. => {
                let delta = delta.saturating_div(1024);
                p = "mB/s";
                format!("^b#222436^ ﯲ  {} {} ^b#222436^\n", delta, p)
            }
            _ => {
                format!("^b#222436^ ﯲ  {} {} ^b#222436^\n", delta, p)
            }
        }
    }
}

fn main() {
    match !env::var("BLOCK_BUTTON").unwrap_or_default().is_empty() {
        true => {
            buttonhandler::button_handler();
        }
        false => (),
    }
    let down_initial: RxBytes = RxBytes::new();

    {
        let dur = Duration::new(1, 0);
        sleep(dur);
    }

    let down_final: RxBytes = RxBytes::new();

    let delta = RxBytes::delta(down_initial, down_final);

    println!("{}", RxBytes::status_text(delta))
}
