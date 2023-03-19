use chrono::{self, Datelike, Months, NaiveDate, Weekday};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    num::ParseIntError,
    str::FromStr,
};

pub trait NthDayExtension {
    fn nth(&self) -> DayOfMonth;
}

impl NthDayExtension for usize {
    fn nth(&self) -> DayOfMonth {
        DayOfMonth::NthDay(*self as u32)
    }
}

impl NthDayExtension for i32 {
    fn nth(&self) -> DayOfMonth {
        DayOfMonth::NthDay(*self as u32)
    }
}

trait LastOfMonthExtension {
    fn last_of_month(&self) -> NaiveDate;
}

impl LastOfMonthExtension for NaiveDate {
    fn last_of_month(&self) -> NaiveDate {
        let mut date = *self;
        let mut next_date = date;
        while date.month() == next_date.month() {
            date = next_date;
            next_date = next_date
                .succ_opt()
                .expect("shouldn't be at the end of time");
        }

        date
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Day {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monday" => Ok(Day::Monday),
            "Tuesday" => Ok(Day::Tuesday),
            "Wednesday" => Ok(Day::Wednesday),
            "Thursday" => Ok(Day::Thursday),
            "Friday" => Ok(Day::Friday),
            "Saturday" => Ok(Day::Saturday),
            "Sunday" => Ok(Day::Sunday),
            _ => Err(format!("{s} is not a valid day")),
        }
    }
}

impl From<Weekday> for Day {
    fn from(value: Weekday) -> Self {
        match value {
            Weekday::Mon => Day::Monday,
            Weekday::Tue => Day::Tuesday,
            Weekday::Wed => Day::Wednesday,
            Weekday::Thu => Day::Thursday,
            Weekday::Fri => Day::Friday,
            Weekday::Sat => Day::Saturday,
            Weekday::Sun => Day::Sunday,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Month {
    January,
    Feburary,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn from_month_num(num: u32) -> Result<Self, String> {
        match num {
            1 => Ok(Month::January),
            2 => Ok(Month::Feburary),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(format!("{num} is not a valid month number")),
        }
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Clone, Copy)]
pub enum DayOfMonth {
    #[default]
    FirstOfMonth,
    NthDay(u32),
}

const FIRST_OF_MONTH_STR: &str = "first";

impl DayOfMonth {
    pub fn nth_day(day: u32) -> Result<Self, String> {
        if day > 28 {
            Err(format!(
                "Cannot go between {day}. Highest date is 28th due to Feburary"
            ))
        } else {
            Ok(DayOfMonth::NthDay(day))
        }
    }
}

impl FromStr for DayOfMonth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            FIRST_OF_MONTH_STR => Ok(DayOfMonth::FirstOfMonth),
            _ => s
                .parse()
                .map_err(|e: ParseIntError| e.to_string())
                .map(DayOfMonth::nth_day)?,
        }
    }
}

impl Display for DayOfMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DayOfMonth::FirstOfMonth => write!(f, "{}", FIRST_OF_MONTH_STR),
            DayOfMonth::NthDay(num) => write!(f, "{}", num),
        }
    }
}

pub struct MontheyBuilder {
    days_to_check: HashSet<Day>,
    start_of_month: DayOfMonth,
    from_date: NaiveDate,
}

pub struct MontheyMonth {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub days: Box<[(Day, u32)]>,
}

pub struct MontheyResult {
    pub months: Box<[MontheyMonth]>,
}

impl MontheyBuilder {
    pub fn from_this_month() -> Self {
        let now = chrono::offset::Local::now();

        Self {
            days_to_check: Default::default(),
            start_of_month: DayOfMonth::default(),
            from_date: now.date_naive().with_day(1).expect("1 is a valid day"),
        }
    }

    pub fn check_day(mut self, day: Day) -> Self {
        self.days_to_check.insert(day);
        self
    }

    pub fn between_date(mut self, date: DayOfMonth) -> Result<Self, String> {
        if let DayOfMonth::NthDay(day) = date {
            if day > 28 {
                return Err(format!(
                    "Cannot go between {day}. Highest date is 28th due to Feburary"
                ));
            }
        }
        self.start_of_month = date;
        Ok(self)
    }

    pub fn for_months(self, months: u32) -> MontheyResult {
        let mut monthey_months = HashMap::new();

        fn get_to_date(date: NaiveDate, between_date: &DayOfMonth) -> NaiveDate {
            match between_date {
                DayOfMonth::FirstOfMonth => date.last_of_month(),
                DayOfMonth::NthDay(day) => date
                    .checked_add_months(Months::new(1))
                    .expect("shouldn't be at the end of time")
                    .with_day(*day)
                    .expect("should be checked")
                    .pred_opt()
                    .expect("shouldn't be at the start of time"),
            }
        }

        let mut date = match self.start_of_month {
            DayOfMonth::FirstOfMonth => {
                self.from_date.with_day(1).expect("1 is always a valid day")
            }
            DayOfMonth::NthDay(day) => self
                .from_date
                .with_day(day)
                .expect("should already be checked"),
        };

        let mut month_counter = 0;

        while month_counter < months {
            let from_date = date;
            let to_date = get_to_date(date, &self.start_of_month);
            let mut days = HashMap::new();

            while date <= to_date {
                for day in self.days_to_check.iter() {
                    let date_day = date.weekday().into();
                    if *day == date_day {
                        if let Some(day_count) = days.get_mut(day) {
                            *day_count += 1;
                        } else {
                            days.insert(*day, 1);
                        }
                    }
                }

                date = date.succ_opt().expect("shouldn't be at the end of time");
            }

            monthey_months.insert(from_date, days);

            month_counter += 1;
        }

        let months = monthey_months
            .into_iter()
            .map(|(from, days)| MontheyMonth {
                from,
                to: get_to_date(from, &self.start_of_month),
                days: days.into_iter().collect(),
            })
            .collect();

        MontheyResult { months }
    }
}
