use chrono::prelude::*;
use libc::c_double;
use argparse::{ArgumentParser, StoreTrue};

extern "C" {
    fn fmod(a: c_double, b: c_double) -> c_double;
}

const LUNAR_DAYS: f64 = 29.53058770576;
const LUNAR_SECONDS: f64 = LUNAR_DAYS * (24.*60.*60.);
const NEW_MOON_2000: i64 = 947180794;

fn get_current_date() -> DateTime<Local> {
    return chrono::Local::now()
}

fn get_timestamp(date: DateTime<Local>) -> i64 {
    return date.timestamp()
}

fn time_since_nm() -> i64 {
    return get_timestamp(get_current_date()) - NEW_MOON_2000
}

fn calculate_lunar_days(time_since_nm: i64) -> f64 {
    let mut current_secs;
    unsafe {
        current_secs = fmod(time_since_nm as f64, LUNAR_SECONDS);
    }
    if current_secs < 0. {
        current_secs += LUNAR_SECONDS;
    }
    let fractional = current_secs as f64 / LUNAR_SECONDS;
    let current_day = fractional * LUNAR_DAYS;
    return current_day;
}

fn is_between(a: f64, b:f64, c:f64) -> bool {
    return a >= b && a < c;
}

fn print_lunar_phase_text(age: f64) -> () {
    if is_between(age, 0., 1.) { println!("New Moon"); }
    if is_between(age, 1., 6.38264692644) { println!("Waxing Crescent"); }
    if is_between(age, 6.38264692644, 8.38264692644) { println!("First Quarter"); }
    if is_between(age, 8.38264692644, 13.76529385288) { println!("Waxing Gibous"); }
    if is_between(age, 13.76529385288, 15.76529385288) { println!("Full Moon"); }
    if is_between(age, 15.76529385288, 21.14794077932) { println!("Waning Gibbous"); }
    if is_between(age, 21.14794077932, 23.14794077932) { println!("Last Quarter"); }
    if is_between(age, 23.14794077932, 28.53058770576) { println!("Waning Crescent"); } 
    if is_between(age, 28.53058770576, 30.) { println!("New Moon"); }
}

fn print_lunar_phase_emoji(age: f64) -> () {
    if is_between(age, 0., 1.) { println!("🌑"); }
    if is_between(age, 1., 6.38264692644) { println!("🌒"); }
    if is_between(age, 6.38264692644, 8.38264692644) { println!("🌓"); }
    if is_between(age, 8.38264692644, 13.76529385288) { println!("🌔"); }
    if is_between(age, 13.76529385288, 15.76529385288) { println!("🌕"); }
    if is_between(age, 15.76529385288, 21.14794077932) { println!("🌖"); }
    if is_between(age, 21.14794077932, 23.14794077932) { println!("🌗"); }
    if is_between(age, 23.14794077932, 28.53058770576) { println!("🌘"); } 
    if is_between(age, 28.53058770576, 30.) { println!("🌑"); }
}
fn main() {
    let mut show_moon: bool = false;
    let mut short_text: bool = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Show the current lunar phase");
        ap.refer(&mut show_moon).add_option(&["-e", "--emoji"], StoreTrue, "Show moon emoji instead of text");
        ap.refer(&mut short_text).add_option(&["-s", "--short-text"], StoreTrue, "Only print short text");
        ap.parse_args_or_exit();
    }

    let time_since_nm = time_since_nm();
    let current_day = calculate_lunar_days(time_since_nm);

    if show_moon {
        print_lunar_phase_emoji(current_day);
    } else {
        if !short_text {
            print!("Today is a ");
        }
        print_lunar_phase_text(current_day);

    }
}
