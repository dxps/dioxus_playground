## A Dioxus example using Axum and its Session features

This example is based on [axum-auth](https://github.com/DioxusLabs/dioxus/tree/main/packages/fullstack/examples/axum-auth) example, and adapted to:

1. Use PostgreSQL database.
2. Have a server function that can use the database.

<br/>

### Prereqs

Besides the standard Rust setup (that includes `cargo`), the followings are the prerequisites for running this project:

-   Docker

-   `psql`

    -   For Ubuntu based distro, see [this page](https://www.postgresql.org/download/linux/ubuntu/).
    -   Basically, use:

        ```shell
        ❯ sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
        ❯ curl -fsSL https://www.postgresql.org/media/keys/ACCC4CF8.asc \
            | sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/postgresql.gpg
        ❯ sudo apt install postgresql-client-16
        ```

-   Dioxus CLI
    -   Run `cargo install dioxus-cli` to install it and thus have `dx` tool available.

<br/>

### Usage

First, start the database (as a Docker container) using `./run_db.sh`.

Run the app using `dx serve --hot-reload` and this will start it _in dev mode_ (it will reload the UI in case RSX code is changed, or rebuild and restart in case of back-end code changes).

Then go to http://localhost:3000/ to play with the result.

Note that the Dioxus server function named `get_user_name()` uses the database to get some data.

<br/>

### Build

Use `dx build --features web` to build it.\
Then use `cargo run --features server` to start it.
