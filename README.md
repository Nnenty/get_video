<h1 align ="center">Usage</h1>

- First clone our repository:
```
git clone https://github.com/Nnenty/get_video
```
## Server
1. Change the [.env](./.env) file if necessary.
2. Go to [server catalog](./server):
```
cd server/
```
3. Run `docker compose`:
``` 
docker compose up --build
```

The server should start listening for incoming connections on your port.

## Client
1. Go to [client catalog](./client):
```
cd client/
```
2. Change [config.toml](./client/config.toml) if you have changed [.env](./.env) file.
3. Run client
```
cargo run
```

Client should get response and save video into current directory.
