# rocket5-react-blueprint
A simple blueprint of a rust rocket.rs 5.x based backend with a react (cra) frontend using the Blockfrost API to interact with the Cardano ecosystem.

## Prerequirements

- [Install Rust](https://www.rust-lang.org/tools/install) on your system
- [Setup npm & node](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- Create a Project on [https://blockfrost.io/](https://blockfrost.io/) using the free or any other plan

## Getting Started

```zh
git clone https://github.com/fabianbormann/rocket5-react-blueprint.git
cd rocket5-react-blueprint
touch blockfrost.toml
```

Edit the blockfrost.toml and fill it with content:

```toml
project_id = <The Project Id from your blockfrost project dashboard e.g. "testnetaXLfzU....">
cardano_network = <Cardano network e.g."https://cardano-testnet.blockfrost.io/api/v0/">
ipfs_network = <IPFS Network e.g. "https://ipfs.blockfrost.io/api/v0">
```

Build the frontend

```
cd frontend
npm i
```

Go back the the project root folder and start the server

```zh
cd ..
cargo run
```

## Testing

The Server should has [launched at http://127.0.0.1:8000](http://127.0.0.1:8000). You can visit the url with your browser to see the react frontend (cra basic app).

Test the GET endpoint with any wallet address (using curl or your browser):
```zsh
curl http://127.0.0.1:8000/address/addr_test1qzm3ygefcyz30h8rm6ruegdv83z80x02yt2s3k3269eaktfwg4z275krplwsjhr48ac37tffucmtw0wm77a3k5g0k4ksnwsx0t
```

Use curl or Insomnia to test the POST endpoint:

```
curl -d '{"name": "example_name", "description": "hello world", "amount": 42}' -H "Content-Type: application/json" -X POST http://localhost:8000/example
```
