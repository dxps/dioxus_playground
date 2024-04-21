## Routing Fullstack Example

<br/>

### Prerequisites

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

<br/>

### Run

Start the TailwindCSS CLI to watch and build the final stylesheet (or use the provided script: `./run_css.sh`):

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

Launch the Dioxus Fullstack app (or use the provided script: `./run_dev.sh`):

```bash
dx serve --platform fullstack --hot-reload
```
