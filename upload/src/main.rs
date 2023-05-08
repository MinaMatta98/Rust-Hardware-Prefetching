use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;
pub mod buttonhandler;

struct TxBytes {
    tx: i64,
}

impl TxBytes {
    fn new() -> Self {
        let file = File::open("/sys/class/net/eno1/statistics/tx_bytes").unwrap();
        let mut up = String::new();
        BufReader::new(file).read_line(&mut up).unwrap();
        Self {
            tx: up.trim().parse::<i64>().unwrap(),
        }
    }

    fn delta(up_initial: Self, up_final: Self) -> i64 {
        (up_final.tx - up_initial.tx) / 1024
    }

    fn status_text(delta: i64) -> () {
        let mut p = "kB/s";

        match delta {
            99.. => {
                let delta = delta.saturating_div(1024);
                p = "mB/s";
                println!("^c#6666ea^^b#222436^ ﯴ  {} {} ^b#222436^\n", delta, p)
            }
            _ => {
                println!("^c#6666ea^^b#222436^ ﯴ  {} {} ^b#222436^\n", delta, p)
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
    let up_initial: TxBytes = TxBytes::new();

    {
        let dur = Duration::new(1, 0);
        sleep(dur);
    }

    let up_final: TxBytes = TxBytes::new();

    let delta = TxBytes::delta(up_initial, up_final);

    TxBytes::status_text(delta)
}
