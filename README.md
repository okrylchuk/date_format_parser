# Date Format Parser

This Rust project implements a date format parser that can parse dates in multiple common formats. The parser is designed to recognize and extract day, month, and year components from date strings and convert them to a standardized `YYYY-MM-DD` format.

## Features

- Supports multiple date formats:
    - `DMY` (e.g., `20/12/2000` for 20th December 2000)
    - `MDY` (e.g., `12-20-2000` for December 20th, 2000)
- Recognizes different separators: `-`, `/`, and `.`.
- Outputs parsed dates in the `YYYY-MM-DD` format.

## Usage

To use the date parser, provide a date string in one of the supported formats. The parser will output the date in `YYYY-MM-DD` format if the input is valid.

## Use Cases

The primary purpose of this parser is to transform dates in varied formats into a consistent `YYYY-MM-DD` format, which is ideal for:

- **Database Storage**: Storing dates in a standard format helps with database indexing, sorting, and querying.
- **Date Calculations and Comparisons**: Standardizing dates to `YYYY-MM-DD` simplifies chronological calculations.
- **User Interface Display**: Converting dates to a consistent format can help ensure that dates are displayed uniformly across an application, improving user experience.
