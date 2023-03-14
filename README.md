[![build](https://github.com/qdegraaf/buic/.github/workflows/ci.yml/badge.svg)](https://github.com/qdegraaf/buic/.github/workflows/ci.yml)

# Overview

`buic` is a command line tool which connects to the APIs of www.buienradar.nl to give you weather information without
having to leave your terminal.

# Installing

// TODO


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
buic weather actuals --station <STATION>

Options:
  -s, --station <STATION>  
  -h, --help               Print help
```
e.g. 
```
buic weather actuals -s Arnhem
```

### weather: forecast
```
buic weather forecast --n-days <N_DAYS>

Options:
  -n, --n-days <N_DAYS>  
  -h, --help             Print help
```
e.g.
```
buic weather forecast -n 2
```
