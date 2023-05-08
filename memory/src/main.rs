use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};
mod buttonhandler;

#[derive(Debug)]
struct Memory {
    mem_total: i32,
    mem_free: i32,
    mem_available: i32,
}

impl Memory {
    fn new() -> Self {
        let mut file = File::open("/proc/meminfo").unwrap();
        let mut cursor = BufReader::new(&mut file);
        let mut buffer = String::new();

        let mut numcount = 0;

        let mut memmory = Memory {
            mem_total: 0,
            mem_free: 0,
            mem_available: 0,
        };

        loop {
            cursor.read_line(&mut buffer).unwrap();
            numcount += 1;

            memmory[numcount] = buffer
                .lines()
                .nth(numcount as usize - 1 as usize)
                .unwrap()
                .trim()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            if numcount.eq(&3) {
                break;
            }
        }
        memmory
    }

    fn memmory_usage(&self) -> i32 {
        ((1.0 - (self.mem_available as f32 / self.mem_total as f32)) * 100.0) as i32
    }

    fn status_text(memmory_usage: i32) -> () {
        let icon = "  ";

        match memmory_usage {
            99..=100 => println!(
                "^c#ff3c4b^ {} {}%^r-55,23,51,5^^b#222436^\n",
                icon, memmory_usage
            ),
            70..=98 => println!(
                "^c#ff3c4b^ {} {}%^r-46,23,55,5^^b#222436^\n",
                icon, memmory_usage
            ),
            55..=69 => println!(
                "^c#FFBF00^ {} {}%^r-46,23,55,5^^b#222436^\n",
                icon, memmory_usage
            ),
            10..=54 => println!(
                "^d^^b#222436^ {} {}%^r-46,23,55,5^^b#222436^\n",
                icon, memmory_usage
            ),
            _ => println!(
                "^d^^b#222436^ {} {}%^r-40,23,41,5^^b#222436^\n",
                icon, memmory_usage
            ),
        }
    }
}

impl Index<i32> for Memory {
    type Output = i32;
    fn index(&self, s: i32) -> &i32 {
        match s {
            1 => &self.mem_total,
            2 => &self.mem_free,
            3 => &self.mem_available,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<i32> for Memory {
    fn index_mut(&mut self, s: i32) -> &mut i32 {
        match s {
            1 => &mut self.mem_total,
            2 => &mut self.mem_free,
            3 => &mut self.mem_available,
            _ => panic!("unknown field: {}", s),
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

    let memmory = Memory::new();
    let memmory_usage = memmory.memmory_usage();
    Memory::status_text(memmory_usage);
}
