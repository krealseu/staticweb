# Samlple static web server


## config file `.env`
```
port=8080    # default 8080
root=static  # default ""
```
if file .env not exists, server will use default value.

## build
```
cargo build --release
```

## usaged
```
staticweb [port]
```
Port will load from file .env if not define.