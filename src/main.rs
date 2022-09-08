use chrono::{Datelike, NaiveDate};
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct DateFormatError;
impl fmt::Display for DateFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid date format")
    }
}

// Implement iterator
struct DateRange {
    start: NaiveDate,
    end: NaiveDate,
}

impl DateRange {
    pub fn new(start: NaiveDate, end: NaiveDate) -> DateRange {
        DateRange { start, end }
    }
}

impl Iterator for DateRange {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<NaiveDate> {
        if self.start.gt(&self.end) {
            None
        } else {
            let res = self.start;
            self.start = self.start.succ();
            Some(res)
        }
    }
}

fn get_day_month_year(d: &str) -> Result<(u32, u32, i32), DateFormatError> {
    let mut result = (0u32, 0u32, 0i32);
    let parts = d.split("/");

    let mut counter = 0u8;
    for p in parts {
        if counter == 0 {
            let number = match p.parse::<u32>() {
                Ok(result) => result,
                Err(_) => return Err(DateFormatError),
            };
            result.0 = number;
        } else if counter == 1 {
            let number = match p.parse::<u32>() {
                Ok(result) => result,
                Err(_) => return Err(DateFormatError),
            };
            result.1 = number;
        } else if counter == 2 {
            let number = match p.parse::<i32>() {
                Ok(result) => result,
                Err(_) => return Err(DateFormatError),
            };
            result.2 = number;
        } else {
            return Err(DateFormatError);
        }

        counter += 1;
    }

    Ok(result)
}

fn str_to_date(t: &str) -> Result<NaiveDate, DateFormatError> {
    let (day, month, year) = match get_day_month_year(t) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };
    Ok(NaiveDate::from_ymd(year, month, day))
}

fn load_excluded_dates() -> Result<HashSet<String>, DateFormatError> {
    let mut exclude_days: HashSet<String> = HashSet::new();

    let file = match File::open("C:/Users/tia/Documents/projects/daycounter/exclude.txt") {
        Ok(result) => result,
        Err(e) => panic!("error opening file {}", e),
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Ok(result) => result,
            Err(e) => panic!("error readingf line {}", e),
        };

        match str_to_date(line.as_str()) {
            Ok(result) => exclude_days.insert(result.to_string()),
            Err(e) => return Err(e),
        };
    }

    Ok(exclude_days)
}

fn main() {
    // inici / fi
    let start = "12/09/2022";
    let end = "10/12/2022";
    // posar nombre de sessions de cada dia [dll, dm, dx, dj, dv, ds, dm]
    let included_week_days: [u32; 7] = [1, 1, 0, 1, 1, 0, 0];

    let start_date = str_to_date(start).unwrap();
    let end_date = str_to_date(end).unwrap();

    // add excluded days from file
    let exclude_days = match load_excluded_dates() {
        Ok(result) => result,
        Err(e) => panic!("Can process excluded days {}", e),
    };

    let period = DateRange::new(start_date, end_date);
    let mut count = 0u32;
    let mut week_count = 0u32;
    let mut last_week = 0u32;

    for d in period {
        if d.iso_week().week() != last_week {
            last_week = d.iso_week().week();
            week_count += 1;
        }

        // skip if d is not and included week day
        if included_week_days[(d.weekday() as usize)] == 0 {
            continue;
        }

        // skip if d is in exclude days.
        if exclude_days.contains(&d.to_string()) {
            continue;
        }

        count += included_week_days[(d.weekday() as usize)];

        // 0 indicates pad with zeros
        // 8 is the target width
        println!("{}/{}/{}", format!("{:02}", d.day()), format!("{:02}", d.month()), d.year());
    }

    println!("\nResum");
    println!("Sessions totals: {}", count);
    println!("Nombre de setmanes: {}\n", week_count);
}
