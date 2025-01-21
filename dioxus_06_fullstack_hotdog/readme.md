# Dioxus 0.6 Fullstack example of HotDog app

A minimal fullstack example of the HotDog sample built with Dioxus 0.6.\
It showcases:

-   On front-end side: how to use components and call server functions.
-   On back-end side: how to implement server functions.

<br/>

## Prerequisites & Setup

The followings are the required tools and steps to have the proper setup for the app to run.

### Front-end related

Note: These are needed during development. In other words, if you don't change any code in the components (within `rsx` blocks), then there is no need to run the Tailwind CSS compiler in parallel with the back-end. However, updates to `tailwindcss` npm package generate updates the final `main.css` file.

1. Install [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm).
2. Install the Tailwind CSS [CLI](https://tailwindcss.com/docs/installation).

### Back-end related

1. Have `rust` installed using [rustup.rs](https://rustup.rs/).
2. Install Dioxus CLI ver. 0.5.7 using `cargo install dioxus-cli@0.5.7`.

<br/>

## Run

-   Start the Tailwind CSS compiler using `./run_css.sh` script.
-   Start the back-end (that includes the front-end pages/views) using `./run-dev.sh`.

<br/>

## Usage

-   Navigate to `http://localhost:3002/`.
-   When you click 'Save', the URL of the image is appended to a `dogs.txt` file that is created at the project root level.
