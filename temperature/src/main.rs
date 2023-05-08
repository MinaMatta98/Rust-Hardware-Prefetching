use std::fs;
mod buttonhandler;
use std::env;

#[derive(Debug)]
struct Temp {
    average: i32,
}

impl Temp {
    fn new() -> Self {
        let mut i = 2;
        let mut dir = String::new();
        let mut temp: i32 = 0;

        match fs::read_dir("/sys/devices/platform/coretemp.0/hwmon/hwmon4/") {
            Ok(_) => dir.push_str("/sys/devices/platform/coretemp.0/hwmon/hwmon4/"),
            Err(_) => dir.push_str("/sys/devices/platform/coretemp.0/hwmon/hwmon5/"),
        };

        while i < 8 {
            temp += &fs::read_to_string(&format!("{}temp{}_input", dir, i))
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap();
            i += 1;
        }

        Temp {
            average: temp / 6000,
        }
    }

    fn status_text(self) -> () {
        let icon = "   ";

        match self.average {
            99..=100 => {
                println!(
                    "  ^c#ff3c4b^^b#222436^{} {} 糖 ^r-65,23,65,5^^b#222436^\n",
                    icon, self.average
                );
            }
            75..=98 => {
                println!(
                    "  ^c#ff3c4b^^b#222436^{} {} 糖 ^r-55,23,55,5^^b#222436^\n",
                    icon, self.average
                );
            }
            55..=74 => {
                println!(
                    "  ^c#FFBF00^^b#222436^{} {} 糖 ^r-55,23,55,5^^b#222436^\n",
                    icon, self.average
                );
            }
            9..=54 => {
                println!(
                    "  ^c#6666ea^^b#222436^{} {} 糖 ^c#51afef^^r-55,23,55,5^^b#222436^\n",
                    icon, self.average,
                );
            }
            _ => {
                println!(
                    "  ^c#6666ea^^b#222436^{} {} 糖 ^c#51afef^^r-35,23,35,5^^b#222436^\n",
                    icon, self.average
                );
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

    let temp = Temp::new();

    temp.status_text();
}
