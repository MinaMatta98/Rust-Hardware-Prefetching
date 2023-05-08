use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;
pub mod buttonhandler;

#[derive(Debug)]
struct CpuData {
    a: f32,
    b: f32,
    c: f32,
    idle: f32,
}

impl CpuData {
    fn new() -> Self {
        let mut cpu_iter_data = String::new();
        let file = File::open("/proc/stat").unwrap();
        let mut reader = BufReader::new(&file);
        reader.read_line(&mut cpu_iter_data).unwrap();
        let mut cpu_iter_data = cpu_iter_data.split_whitespace();
        Self {
            a: cpu_iter_data.nth(1).unwrap().parse::<f32>().unwrap(),
            b: cpu_iter_data.next().unwrap().parse::<f32>().unwrap(),
            c: cpu_iter_data.next().unwrap().parse::<f32>().unwrap(),
            idle: cpu_iter_data.next().unwrap().parse::<f32>().unwrap(),
        }
    }

    fn cpu_sum(&self) -> f32 {
        self.a + self.b + self.c + self.idle
    }

    fn cpu_usage(cpu_data_initial: Self, cpu_data_final: Self) -> i32 {
        ((cpu_data_final.cpu_sum() - cpu_data_initial.cpu_sum())
            - (cpu_data_final.idle - cpu_data_initial.idle)) as i32
            * 100
            / (cpu_data_final.cpu_sum() - cpu_data_initial.cpu_sum()) as i32
    }

    fn status_text(cpu_usage: i32) -> () {
        let icon = " ^b#222436^  ";

        match cpu_usage {
            99..=100 => {
                println!(
                    "^b#222436^ ^c#ff3c4b^{} {}%^b#222436^ ^c#ff3c4b^^r-54,23,51,5^^b#222436^
",
                    icon, cpu_usage
                )
            }
            60..=98 => {
                println!(
                    "^b#222436^ ^c#ff3c4b^{} {}%^b#222436^ ^c#ff3c4b^^r-49,23,46,5^^b#222436^
",
                    icon, cpu_usage
                )
            }
            30..=59 => {
                println!(
                    "^b#222436^ ^c#FFBF00^{} {}%^b#222436^ ^c#FFBF00^^r-49,23,46,5^^b#222436^
",
                    icon, cpu_usage
                )
            }
            10..=29 => {
                println!(
                    "^b#222436^ {} {}%^b#222436^ ^c#51afef^^r-49,23,46,5^^b#222436^
",
                    icon, cpu_usage
                )
            }
            _ => println!(
                "^b#222436^ {} {}%^b#222436^ ^c#51afef^^r-42,23,39,5^
",
                icon, cpu_usage
            ),
        }
    }
}

fn main() -> () {
    if !env::var("BLOCK_BUTTON").unwrap_or_default().is_empty() {
        buttonhandler::button_handler();
    }

    let cpu_data_initial = CpuData::new();

    sleep(Duration::from_millis(300));

    let cpu_data_final = CpuData::new();

    let cpu_usage = CpuData::cpu_usage(cpu_data_initial, cpu_data_final);

    CpuData::status_text(cpu_usage);
}
