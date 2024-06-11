# Workers axum turso boilerplate

This is a boilerplate for a web server using [axum](https://github.com/tokio-rs/axum) and [turso](https://turso.tech) run on Cloudflare [Workers](https://workers.cloudflare.com/).

### What is included in this boilerplate:

- [Axum 0.7.5](https://github.com/tokio-rs/axum) is a web framework based on hyper and tower.
- [worker-rs 0.3.0](https://crates.io/crates/worker/0.3.0)
- [Turso](https://turso.tech) is a database library for Rust.
- [libsql](https://github.com/tursodatabase/libsql) a turso client for Rust. But need use custom version in this project.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Nodejs 20.x](https://nodejs.org/en/download/)
- [Wrangler](https://developers.cloudflare.com/workers/cli-wrangler/install-update)

### Optional tools

Use [asdf](https://asdf-vm.com/guide/getting-started.html) to manage the versions of the tools. Install asdf plugin nodejs, rust.

## Usage

For local development.
Follow the instructions in the [Wrangler](https://developers.cloudflare.com/workers/cli-wrangler/install-update) documentation to check news for wrangler.

1. Need install dependencies

```bash
# init and update git submodules
git submodule init
git submodule update

# install nodejs dependencies
npm install

# optional pnpm install

# install rust dependencies
cargo update
```

2. Run the server

```bash
# run the server
npm start
# or use pnpm start
```

3. Open the browser and go to [http://localhost:8787](http://localhost:8787)

## Deployment

Please check the [Wrangler deploy](https://developers.cloudflare.com/workers/wrangler/commands/#deploy) documentation for more information.

Quick steps:
```bash
npm run deploy
```

## Detail for Worker on Rust

Source and readme: https://github.com/cloudflare/workers-rs
