<h1 align ="center">Usage</h1>

## Server
```
cd server_rust/
```

1. Specify env `PORT`
```
export PORT=8000
```
2. Run server
```
cargo run
```

The server should start listening for incoming connections on your port.

## Client
```
cd get_video_rust/
```

1. Specify the port you specified in the server
```
export PORT=8000
```
2. Run client
```
cargo run
```

Client should get response and save video into current directory.