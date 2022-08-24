use std::env::args;
use std::process::exit;

use tzdb::{local_tz, now, time_zone, tz_by_name, TZ_NAMES};

pub fn main() -> Result<(), now::NowError> {
    let mut args = args().into_iter().fuse();
    let exe = args.next();
    let exe = exe.as_deref().unwrap_or("current-time");
    let argument = args.next();

    if matches!(argument.as_deref(), Some("-l" | "--list")) {
        let mut line = String::with_capacity(80);
        for tz_name in TZ_NAMES {
            if line.len() + 2 + tz_name.len() >= 80 {
                println!("{},", line);
                line.clear();
            }
            if !line.is_empty() {
                line.push_str(", ");
            }
            line.push_str(tz_name);
        }
        if !line.is_empty() {
            println!("{}", line);
        }
        return Ok(());
    }

    let timezone = if let Some(argument) = argument {
        match tz_by_name(&argument) {
            Some(timezone) => timezone,
            None => {
                eprintln!("No such time zone found in database: {:?}", argument);
                eprintln!("To see a list of all known time zones run: {} --list", exe);
                exit(1);
            },
        }
    } else {
        eprintln!("No time zone selected, defaulting to the system time zone.");
        eprintln!("To see a list of all known time zones run: {} --list", exe);
        eprintln!();
        local_tz().unwrap_or(time_zone::UTC)
    };

    let dt = now::in_tz(timezone)?;
    let dow = match DOW.get(dt.week_day() as usize) {
        Some(dow) => *dow,
        None => unreachable!("Impossible week_day: {}", dt.week_day()),
    };
    let month = match MONTH.get(dt.month() as usize) {
        Some(month) => *month,
        None => unreachable!("Impossible month: {}", dt.month()),
    };
    println!(
        "In the time zone {}:",
        dt.local_time_type().time_zone_designation(),
    );
    println!(
        "Today is {}, {} the {}{}.",
        dow,
        month,
        dt.month_day(),
        suffix(dt.month_day() as _)
    );
    println!(
        "This is the {}{} day of the year {}.",
        1 + dt.year_day(),
        suffix((1 + dt.year_day()) as _),
        dt.year(),
    );
    println!(
        "Now it is {:02}:{:02}:{:02}.",
        dt.hour(),
        dt.minute(),
        dt.second(),
    );
    println!("{}", dt);

    Ok(())
}

fn suffix(index: usize) -> &'static str {
    match (index % 100, index % 10) {
        (10..=19, _) => "th",
        (_, 1) => "st",
        (_, 2) => "nd",
        (_, 3) => "rd",
        _ => "th",
    }
}

const DOW: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

const MONTH: [&str; 13] = [
    "",
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
