# BI Helper

These small tools were created for a BI project, where we were asked to analyse 
the AdventureWorks fictive business.

You can find two CSV generator.

One that generates some time data (`src/bin/time.rs`).

The other (`src/bin/forecast.rs`) is more complex and do some web scraping to retrieve weather data in 
the www.ncei.noaa.gov database.

## How to use

You must have rust installed on your machine : follow instructions [here](https://www.rust-lang.org/tools/install)

Then you can run the forecast script with
```shell
cargo run
```

And the time script with
```shell
cargo run --bin=time
```
