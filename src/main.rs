use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use regex::Regex;

#[macro_use]
extern crate lazy_static;
extern crate regex;

fn read_ambient_light() -> u8 {
    let path = "/sys/devices/platform/applesmc.768/light";

    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open light sensor file: {}", e),
    };

    let mut buffer = String::new();
    match f.read_to_string(&mut buffer) {
        Ok(f) => f,
        Err(e) => panic!("Failed to read from light sensor file: {}", e),
    };

    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\((\d+),\d+\)").unwrap();
    }
    let light = match RE.captures(&buffer).unwrap().at(1) {
        Some(x) => x,
        None    => panic!("Bogus value read from light sensor!"),
    };

    let ulight = match light.parse() {
        Ok(i) => i,
        Err(e) => panic!("This should never happen: {}", e),
    };
    return ulight;
}

fn set_backlight(value: u8) {
    let path = "/sys/class/leds/smc::kbd_backlight/brightness";

    let mut f = match File::create(path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open backlight file: {}", e),
    };

    println!("Setting keyboard backlight to {}", value);
    match write!(f, "{}", value) {
        Ok(r) => r,
        Err(e) => panic!("Failed to write backlight value into file: {}", e),
    };
}

pub fn main() {
    loop {
        let ambient = read_ambient_light();
        println!("Light sensor reports a light value of {}", ambient);

        // XXX: Use a smarter algorithm here:
        // * Usual ambient values linger between 0..3.
        // * Backlight of 80 is the strongest ever needed.
        let backlight = 255 - ambient;

        set_backlight(backlight);

        sleep(Duration::from_millis(100));
    }
}
