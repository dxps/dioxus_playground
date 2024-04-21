## Dioxus Fullstack example with routing and server config

<br/>

### Prereqs

1. Install [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm).
2. Install the Tailwind CSS [CLI](https://tailwindcss.com/docs/installation).
3. Run the following command in the root of the project to start the Tailwind CSS compiler:<br/>
   `npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch` <br/>
   or use `./run_css.sh` provided script.

<br/>

### Run

Launch the Dioxus Fullstack app using `dx serve --platform fullstack` or use `./run-dev.sh` provided script.

In case of compilation error that remain hidden behind Dioxus CLI, run `cargo check --features server,web` to reveal them.
