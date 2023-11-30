# Advent of Code 2023

Hello, sailor! This year, I'm solving it with Rust.

## Routine

Star a day with the following:
```bash
$ ./day
```

This will bootstrap today's solution and download the inputs.
Template is contained in `boilerplate.rs`.

If you skipped a day, you can still do it via:
```bash
$ ./day <day-number>
```

## Setup

To download task inputs, the session cookie is required.
You can get one inspecting the headers of AOC website response,
when you are logged in.

Day script looks for it in `.env` file, in the following form:

```bash
KEY=<long alphanumeric string you have in your aoc cookies>
```
