# Monthey
Small library to calculate how many mon(th)ey days in a month(ey)

Figures out how many of a given weekday during each month eg How many fridays for the next 3 months

Optionally can go from a given date eg. 10th to the 9th inclusive.

I get paid monthly, not on the 1st, and so the number of rent/grocery days changes per month.

## Options

### Months
`--months -m <months>`
the number of months from the current month to check.

eg. `--months 3`

### Between
`--between <between>`
Which day of the month to check between

options:
- first: Checks between the first of the month and the last of the month. Only days within each month are included
- <date>: Checks between the given date of the month and the date previous of the next month. Only values up to 28 are valid so that each month can be checked

### Day
`--day Tuesday --day Friday`
Which days of the week to check. Pass multiple days to check multiple weekdays

## Examples
`monthey --months 3 --between 8 --day Tuesday --day Friday`

Checks for the following 3 months (including this month) how many Tuesdays and Fridays between the 8th of the month and the 7th of the next month.

Run on the 20th March 2023 produces
```
8 March 2023 to 7 April 2023 contains:
        4 Tuesday(s)
        5 Friday(s)
8 April 2023 to 7 May 2023 contains:
        4 Tuesday(s)
        4 Friday(s)
8 May 2023 to 7 June 2023 contains:
        5 Tuesday(s)
        4 Friday(s)
```

`monthey --months 2 --between first --day Monday`
Checks for the following 2 months how many Mondays in each month

Run on the 20th March 2023 produces
```
1 March 2023 to 31 March 2023 contains:
        4 Monday(s)
1 April 2023 to 30 April 2023 contains:
        4 Monday(s)
```

## Todo
- tests
- clap
