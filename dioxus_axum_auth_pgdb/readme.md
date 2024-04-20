## A Dioxus example using Axum and its Session features

This example is based on [axum-auth](https://github.com/DioxusLabs/dioxus/tree/main/packages/fullstack/examples/axum-auth) example, adapted to use PostgreSQL database.

<br/>

### Usage

Run it using `dx serve --hot-reload` and this will start it in "dev" mode (it will reload the UI in case RSX code is changed or rebuild and restart in case of back-end code changes).

Then go to http://localhost:3000/ to play with the result.

<br/>

### Build

Use `dx build --features web` to build it.\
Then use `cargo run --features server` to start it.
