```
REDIS-TOOLS 0.1.0
A CLI tool to send commands to redis

USAGE:
    redis-cli.exe [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -l, --live       runs commands for every new line found in stdin.
    -V, --version    Prints version information

OPTIONS:
    -u, --uri <uri>
```
Example;
```
"keys *" | redis-cli
```
Or;
```

redis-cli --live
keys *
```
