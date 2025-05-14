use chrono::{DateTime, Datelike, Duration, Local, NaiveDate};

#[derive(Debug)]
struct PublicHoliday {
    name: String,
    date: NaiveDate,
}

fn calculate_easter(year: i32) -> NaiveDate {
    let aa = year % 19;
    let bb = year / 100;
    let cc = year % 100;
    let dd = bb / 4;
    let ee = bb % 4;
    let gg = ((8 * bb) + 13) / 25;
    let hh = ((19 * aa) + bb - dd - gg + 15) % 30;
    let ii = cc / 4;
    let kk = cc % 4;
    let ll = (32 + (2 * ee) + (2 * ii) - hh - kk) % 7;
    let mm = (aa + (11 * hh) + (19 * ll)) / 433;
    let month = ((hh + ll - (7 * mm) + 90) / 25).wrapping_abs();
    let day = ((hh + ll - (7 * mm) + (33 * month) + 19) % 32).wrapping_abs();
    NaiveDate::from_ymd(year, month as u32, day as u32)
}

fn is_weekend(date: NaiveDate) -> bool {
    date.weekday().number_from_monday() > 5
}

fn is_public_holiday(holidays: &[PublicHoliday], date: NaiveDate) -> bool {
    for p in holidays {
        if p.date == date {
            return true;
        }
    }
    false
}

fn public_holiday(holidays: &[PublicHoliday], date: NaiveDate) -> Option<&PublicHoliday> {
    let mut found: Option<&PublicHoliday> = None;
    for p in holidays {
        if p.date == date {
            found = Some(p);
            break;
        }
    }
    found
}

fn calc_pub(year: i32) -> Vec<PublicHoliday> {
    let pasen: NaiveDate = calculate_easter(year);
    let v: Vec<PublicHoliday> = vec![
        PublicHoliday {
            name: "Eerste Paasdag".to_string(),
            date: pasen,
        },
        PublicHoliday {
            name: "Nieuwjaarsdag".to_string(),
            date: NaiveDate::from_ymd(year, 1, 1),
        },
        PublicHoliday {
            name: "Goede Vrijdag".to_string(),
            date: (pasen - Duration::days(2)),
        },
        PublicHoliday {
            name: "Tweede Paasdag".to_string(),
            date: (pasen + Duration::days(1)),
        },
        PublicHoliday {
            name: "Koningsdag".to_string(),
            date: NaiveDate::from_ymd(year, 4, 27),
        },
        PublicHoliday {
            name: "Bevrijdingsdag".to_string(),
            date: NaiveDate::from_ymd(year, 5, 5),
        },
        PublicHoliday {
            name: "Hemelvaartsdag".to_string(),
            date: pasen + Duration::days(39),
        },
        PublicHoliday {
            name: "Eerste Pinksterdag".to_string(),
            date: pasen + Duration::days(49),
        },
        PublicHoliday {
            name: "Tweede Pinksterdag".to_string(),
            date: pasen + Duration::days(50),
        },
        PublicHoliday {
            name: "Eerste Kerstdag".to_string(),
            date: NaiveDate::from_ymd(year, 12, 25),
        },
        PublicHoliday {
            name: "Tweede Kerstdag".to_string(),
            date: NaiveDate::from_ymd(year, 12, 26),
        },
    ];
    v
}

fn is_leap(year: i32) -> bool {
    let factor = |x| year % x == 0;
    factor(4) && (!factor(100) || factor(400))
}

fn year_days(year: i32) -> i32 {
    match is_leap(year) {
        true => 366,
        false => 365,
    }
}

fn make_day(year_holidays: &Vec<PublicHoliday>, curdate: NaiveDate) {
    println!("Today Overview");
    print!("{} ", &curdate);

    if is_public_holiday(&year_holidays, curdate) {
        match public_holiday(&year_holidays, curdate) {
            Some(x) => print!("| {} ", x.name),
            None => print!("Nah"),
        }
    }
    if is_weekend(curdate) {
        print!("| Weekend")
    }
    println!();
}

fn make_calendar(year_holidays: &Vec<PublicHoliday>, year: i32) {
    let mut my_date = NaiveDate::from_ymd(year, 1, 1);
    println!("Year Overview");
    for _i in 0..year_days(my_date.year()) {
        print!("{} ", &my_date);
        if is_public_holiday(&year_holidays, my_date) {
            match public_holiday(&year_holidays, my_date) {
                Some(x) => print!("| {} ", x.name),
                None => print!("Nah"),
            }
        }
        if is_weekend(my_date) {
            print!("| Weekend")
        }
        println!();
        my_date += Duration::days(1);
    }
}

fn main() {
    let now: DateTime<Local> = Local::now();
    let year = 2024;
    let v: Vec<PublicHoliday> = calc_pub(year);
    let curdate = NaiveDate::from_ymd(now.year(), now.month(), now.day());

    make_day(&v, curdate);
    make_calendar(&v, year);
}
