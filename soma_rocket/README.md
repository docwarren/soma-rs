# GHR API Server

This is a simple API server for the GHR (Gene Health Records) project. It provides endpoints to access gene information and coordinates.

## Endpoints

- `GET /`: Returns a welcome message.
- `GET /genes/<gene>`: Returns a list of genes for the specified gene.
- `GET /genes/coordinates/<gene>`: Returns the coordinates for the specified gene.
- `POST /search`: Searches for features. Args: `path`, `coordinates`

## Running

To run the server, use the following command:

```bash
cargo run
```

## Run in development mode
Ensure you have cargo watch installed
```
cargo install cargo-watch
```
Then run in watch mode
```bash
cargo watch -x run
```