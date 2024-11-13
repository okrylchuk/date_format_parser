# date_format_parser

Link: https://crates.io/crates/date_format_parser

Date Format Parser project is designed to parse date and date-time strings into a standard ISO 8601 format. It supports multiple date formats, optional time components, and flexible separators.

## Features

- Parse dates and date-times in multiple formats:
  - Year-Month-Day (e.g., 2000-12-20).
  - Day-Month-Year (e.g., 20.12.2000).
  - Month-Day-Year (e.g., 12/20/2000).
- Support for optional time components:
  - Time in 24-hour format (e.g., 14:30:45.123).
  - Time with optional seconds and milliseconds.
- Handles common separators:
  - Dashes (-), slashes (/), and dots (.).
- Outputs parsed results in ISO 8601 format:
  - Dates: YYYY-MM-DD
  - Date-times: YYYY-MM-DDTHH:MM:SS[.sss]
- User-friendly CLI interface with commands for:
  - Parsing input.
  - Displaying help information.
  - Showing credits.

## Usage

### Running the CLI

```rust
cargo run -- <input>
```
Replace `<input>` with a date or date-time string.

### Commands

1. Help
```rust
cargo -- --help
```
Displays usage information.

2. Credits:
```rust
cargo run -- --credits
```
Shows project credits.

3. Parsing a Date or Date-Time:
```rust
cargo run -- "2000/12/20 14:30:45.123"
```
Parses the given input and outputs the result in ISO 8601 format:
```rust
"2000-12-20T14:30:45.123"
```

## Supported Input Formats

### Date Formats
1. Year-Month-Day:
 - Example: 2000-12-20
 - Separator: "-"
2. Day-Month-Year:
 - Example: 20.12.2000
 - Separator: "."
3. Month-Day-Year:
 - Example: 12/20/2000
 - Separator: "/"

### Time Formats
1. 24-hour time:
 - HH:MM:SS[.sss]
 - Example: 14:30:45.123
2. Optional components:
 - Without seconds: HH:MM
 - Without milliseconds: HH:MM:SS

### Combined Date-Time
Dates can optionally include times, separated by space:
 - Example: 2000-12-20 14:30:45.123

## Parsing Process (Technical Description)
The parsing process is built on a robust Pest grammar defined in grammar.pest. Here’s how the tool parses the input:

### Input Validation
   The input string is validated against a custom grammar defined in the Pest library. The grammar supports:

 - Date components (year, month, day).
 - Time components (hour, minute, second, millisecond).
 - Flexible separators (-, /, .).

### Grammar
The grammar uses rules to define valid date and time formats:

```rust
date_time = { date ~ (space ~ time)? }

date = { year_month_day | day_month_year | month_day_year }

year_month_day = { year ~ separator ~ month ~ separator ~ day }
day_month_year = { day ~ separator ~ month ~ separator ~ year }
month_day_year = { month ~ separator ~ day ~ separator ~ year }

year = { '1'..'9' ~ ASCII_DIGIT ~ ASCII_DIGIT ~ ASCII_DIGIT }
month = { ("0" ~ '1'..'9') | ("1" ~ '0'..'2') }
day = { ("0" ~ '1'..'9') | ('1'..'2' ~ ASCII_DIGIT) | ("3" ~ '0'..'1') }

time = { hour_24 ~ ":" ~ minute ~ (":" ~ second ~ ("." ~ millisecond)?)? }

hour_24 = { "0" ~ ASCII_DIGIT | "1" ~ ASCII_DIGIT | "2" ~ ('0'..'3') }

minute = { ('0'..'5' ~ ASCII_DIGIT) }
second = { ('0'..'5' ~ ASCII_DIGIT) }
millisecond = { ASCII_DIGIT ~ ASCII_DIGIT ~ ASCII_DIGIT }

separator = { "-" | "/" | "." }
space = { " " }
```
1. Top-Level Rule (date_time):
   - Parses a date.
   - Optionally parses time if preceded by a single space.
2. Date Formats:
   - year_month_day: Matches YYYY-MM-DD.
   - day_month_year: Matches DD.MM.YYYY.
   - month_day_year: Matches MM/DD/YYYY.
3. Time Formats:
   - Matches time in 24-hour format.
   - Handles optional seconds and milliseconds.
4. Separators and Spaces:
   - Dashes (-), slashes (/), dots (.), and spaces are supported.

### Parsing Logic
1. The input string is parsed into components (date and time if present) using the Pest library.
2. The components are validated against the grammar and extracted:
   - Year, month, and day for dates.
   - Hour, minute, second, and millisecond for times.
3. The components are assembled into an ISO 8601 formatted string.

## Use Cases

1. Standardizing Dates and Times:
   - Convert diverse date-time formats into a unified ISO 8601 format.
2. Data Cleaning:
   - Pre-process date-time data from files for further analysis.
3. Log Parsing:
   - Normalize timestamps in application logs for consistent formatting.

## License
This project is licensed under the MIT License.