pub fn get_datetime() -> String {
    let now = std::time::SystemTime::now();
    let secs = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let tm = unix_to_utc(secs);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        tm.year, tm.month, tm.day, tm.hour, tm.min, tm.sec
    )
}

struct Tm {
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    min: u8,
    sec: u8,
}

fn unix_to_utc(mut secs: u64) -> Tm {
    let sec = (secs % 60) as u8;
    secs /= 60;
    let min = (secs % 60) as u8;
    secs /= 60;
    let hour = (secs % 24) as u8;
    secs /= 24;
    let mut year = 1970;
    let mut day_count = secs as i64;

    while day_count >= days_in_year(year) as i64 {
        day_count -= days_in_year(year) as i64;
        year += 1;
    }

    let mut month: u8 = 1;
    while day_count >= days_in_month(year, month) as i64 {
        day_count -= days_in_month(year, month) as i64;
        month += 1;
    }

    let day = (day_count + 1) as u8;
    Tm {
        year,
        month,
        day,
        hour,
        min,
        sec,
    }
}

fn days_in_year(y: i32) -> u16 {
    if (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0) {
        366
    } else {
        365
    }
}

fn days_in_month(y: i32, m: u8) -> u16 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if days_in_year(y) == 366 {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}
