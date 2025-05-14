use chrono::{DateTime, Local, NaiveDate, Datelike, Duration};
use std::collections::HashMap;




fn calculate_easter(year: i32) -> NaiveDate {
    let a = year % 19 ;
    let b = year / 100 ;
    let c = year % 100 ;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = ((19 * a) + b - d- g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i -h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = ((h + l - 7 * m + 114) / 31).wrapping_abs() as u32;
    let day = (((h + l - 7 * m + 114) % 31 ) + 1).wrapping_abs() as u32;
    NaiveDate::from_ymd(year, month, day)
}



fn main() {

    let now: DateTime<Local> = Local::now();

    let curdate = NaiveDate::from_ymd(2021, 1, 1);
    let _curdate = NaiveDate::from_ymd(now.year(), now.month(), now.day());

    let year = curdate.year();
    let eerste_paasdag: NaiveDate = calculate_easter(year);    

    let nieuwjaarsdag = NaiveDate::from_ymd(year, 1, 1);
    let tweede_paasdag: NaiveDate = eerste_paasdag + Duration::days(1);
    let koningsdag = NaiveDate::from_ymd(year, 4, 27);
    let hemelvaartsdag: NaiveDate = eerste_paasdag + Duration::days(39);
    let eerste_pinksterdag: NaiveDate = eerste_paasdag + Duration::days(49);
    let tweede_pinksterdag: NaiveDate = eerste_paasdag + Duration::days(50);
    let eerste_kerstdag = NaiveDate::from_ymd(year, 12, 25);
    let tweede_kerstdag = NaiveDate::from_ymd(year, 12, 26);

    let mut x = HashMap::new();

    x.insert(nieuwjaarsdag, "Nieuwjaarsdag".to_string());
    x.insert(eerste_paasdag, "Eerste Paasdag".to_string());
    x.insert(tweede_paasdag, "Tweede Paasdag".to_string());
    x.insert(koningsdag, "Koningsdag".to_string());
    x.insert(hemelvaartsdag, "Hemelvaartsdag".to_string());
    x.insert(eerste_pinksterdag, "Eerste Pinksterdag".to_string());
    x.insert(tweede_pinksterdag, "Tweede Pinksterdag".to_string());
    x.insert(eerste_kerstdag, "Eerste Kerstdag".to_string());
    x.insert(tweede_kerstdag, "Tweede Kerstdag".to_string());
    

    let mut y = HashMap::new();

    y.insert("Nieuwjaarsdag".to_string(), nieuwjaarsdag);
    y.insert("Eerste Paasdag".to_string(), eerste_paasdag);
    y.insert("Tweede Paasdag".to_string(), tweede_paasdag);
    y.insert("Koningsdag".to_string(), koningsdag);
    y.insert("Hemelvaartsdag".to_string(), hemelvaartsdag);
    y.insert("Eerste Pinksterdag".to_string(), eerste_pinksterdag);
    y.insert("Tweede Pinksterdag".to_string(), tweede_pinksterdag);
    y.insert("Eerste Kerstdag".to_string(), eerste_kerstdag);
    y.insert("Tweede Kerstdag".to_string(), tweede_kerstdag);


    for (key, val) in x.iter() {
        println!("key: {} val: {}", key, val);
    }
    
    for val in y.values() {
        println!("{}", val);
    }

    let value_exists = y.values().any(|&x| x == curdate);
    println!("{}", value_exists);

    let key_exists = x.keys().any(|&x| x == curdate);
    println!("{}", key_exists);

    match x.get(&curdate) {
        Some(day) => println!("{} is: {}", &curdate, day),
        None => println!("{} is unreviewed.", &curdate)
    }

    let mut my_date = curdate;
    for _i in 0..31 {
        print!("{}:", &my_date);
        match x.get(&my_date) {
            Some(day) => print!("{}", day),
            None => if my_date.weekday().number_from_monday() > 5 {
                print!("Weekend")
            }
        }
        println!();
        my_date = my_date + Duration::days(1);
    }

}
