[![CI](https://github.com/qdegraaf/buic/actions/workflows/ci.yml/badge.svg)](https://github.com/qdegraaf/buic/actions/workflows/ci.yml)
[![Release](https://github.com/qdegraaf/buic/actions/workflows/release.yml/badge.svg)](https://github.com/qdegraaf/buic/actions/workflows/release.yml)
# Overview

`buic` is a command line tool which connects to the APIs of www.buienradar.nl to give you weather information without
having to leave your terminal.

# Installing

### Build from source
Download the source code, either by cloning the repo or downloading a zip with the code from 
[Releases](https://github.com/qdegraaf/buic/releases/). Then run 
```bash
cargo build
```

### Download binary
Find your OS under [Releases](https://github.com/qdegraaf/buic/releases/) and download the version you want. Unpack and
move the binary into your $PATH. 


# Usage
```
buic [OPTIONS] <COMMAND>

Commands:
  rain     Get rain data 2h into the future for any lat/lon pair in NL/BE. Defaults to Utrecht
  weather  Get more detailed weather info from the buienradar.nl JSON API
  help     Print this message or the help of the given subcommand(s)

Options:
  -o, --output <OUTPUT>      Output file, stdout if not present
  -f, --filetype <FILETYPE>  Output filetype [possible values: csv, json]
  -h, --help                 Print help

```

Right now the CLI supports two main modes:
- `rain`: Get the rain forecast 2h into the future for any LAT/LON pair in NL/BE
- `weather`: Get the actuals of any measuring station in NL/BE based on name or get a nationwide forecast up 
to 5 days into the future

### rain
```
Get rain data 2h into the future for any lat/lon pair in NL/BE. Defaults to Utrecht

buic rain [OPTIONS]

Options:
      --latitude <LATITUDE>    Latitude> [default: 52.0907]
      --longitude <LONGITUDE>  Longitude [default: 5.1214]
  -h, --help                   Print help
```
e.g. 
```
buic rain --latitude 51.1 --longitude 4.9231
```
### weather: actuals
```
Get actual weather data from weather stations around the Benelux

Usage: buic weather actuals --station <STATION>

Options:
  -s, --station <STATION>  Name of the weather station for which you want to get actual weather data
  -h, --help               Print help
```
e.g. 
```
buic weather actuals -s Arnhem
```

### weather: forecast
```
Get the country wide forecast for 1-5 days into the future

Usage: buic weather forecast --n-days <N_DAYS>

Options:
  -n, --n-days <N_DAYS>  Number of days into the future to get forecast for
  -h, --help             Print help
```
e.g.
```
buic weather forecast -n 2
```
