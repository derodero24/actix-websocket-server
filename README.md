# Actix WebSocket Server

## Run

Start Server

```shell
$ cargo run
```

Connect client

```shell
$ wscat -c ws://localhost:8080
> message
< message
> close
Disconnected (code: 1000, reason: "Catch close operation.")
```
