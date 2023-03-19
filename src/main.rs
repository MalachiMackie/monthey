use std::collections::HashMap;

use chrono::prelude::*;
use clap::{Arg, ArgAction, Command};
use iter_tools::Itertools;
use monthey::{Day, DayOfMonth, MontheyBuilder};
use num_traits::FromPrimitive;

const COMMAND_NAME: &str = "monthey";
const MONTHS_ARG: &str = "months";
const BETWEEN_ARG: &str = "between";
const DAYS_ARG: &str = "day";

fn main() {
    let matches = Command::new(COMMAND_NAME)
        .about("calculates given weekday occurrences each month between dates")
        .args([
            Arg::new(MONTHS_ARG)
                .long(MONTHS_ARG)
                .short('m')
                .default_value("3")
                .value_parser(clap::value_parser!(u32)),
            Arg::new(BETWEEN_ARG)
                .long(BETWEEN_ARG)
                .short('b')
                .default_value("first")
                .value_parser(clap::value_parser!(DayOfMonth)),
            Arg::new(DAYS_ARG)
                .long(DAYS_ARG)
                .short('d')
                .action(ArgAction::Append)
                .required(true)
                .value_parser(clap::value_parser!(Day)),
        ])
        .get_matches();

    let months: u32 = matches
        .get_one(MONTHS_ARG)
        .copied()
        .expect("default value should be set");

    let between_result: &DayOfMonth = matches
        .get_one(BETWEEN_ARG)
        .expect("default value should be set");

    let days: Vec<Day> = matches
        .get_many(DAYS_ARG)
        .expect("default value should be set")
        .copied()
        .collect();

    let mut builder = MontheyBuilder::from_this_month();

    for day in days.into_iter() {
        builder = builder.check_day(day);
    }

    let result = builder
        .between_date(*between_result)
        .unwrap()
        .for_months(months);

    let display_names: HashMap<Day, &'static str> = [
        // (Day::Thursday, "Rent Day"),
        // (Day::Saturday, "Grocery shopping day"),
    ]
    .into_iter()
    .collect();

    for month in result.months.iter().sorted_by_key(|month| month.from) {
        println!(
            "{} to {} contains:",
            format_date(month.from),
            format_date(month.to)
        );
        for (day, num_days) in month
            .days
            .iter()
            .sorted_by(|(day_a, _), (day_b, _)| Ord::cmp(day_a, day_b))
        {
            if let Some(display_name) = display_names.get(day) {
                println!("\t{num_days} {display_name}(s)");
            } else {
                println!("\t{num_days} {day}(s)");
            }
        }
    }
}

fn format_date(date: NaiveDate) -> String {
    format!(
        "{} {:?} {}",
        date.day(),
        Month::from_u32(date.month()).unwrap(),
        date.year()
    )
}
