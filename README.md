<h1 align ="center">Usage</h1>

- First clone our repository:
```
git clone https://github.com/Nnenty/get_video
```
## Server
1. Change the port in the `.env` file if necessary.
2. Go to `server catalog`:
```
cd server/
```
3. Run `docker compose`:
``` 
docker compose up --build
```

The server should start listening for incoming connections on your port.

## Client
1. Change the `config.toml` file if necessary.
2. Go to `client catalog`:
```
cd client/
```

3. Run client
```
cargo run
```

Client should get response and save video into current directory.