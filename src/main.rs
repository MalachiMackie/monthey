use std::collections::HashMap;

use chrono::prelude::*;
use iter_tools::Itertools;
use monthey::{Day, MontheyBuilder, NthDayExtension};
use num_traits::FromPrimitive;

fn main() {
    let result = MontheyBuilder::from_this_month()
        .check_day(Day::Thursday)
        .check_day(Day::Saturday)
        .between_date(10.nth())
        .unwrap()
        .for_months(3);

    let display_names: HashMap<_, _> = [
        (Day::Thursday, "Rent Day"),
        (Day::Saturday, "Grocery shopping day"),
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
