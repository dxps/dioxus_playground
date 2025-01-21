#!/bin/sh

## Note: `dx` version 0.6.x must be used.

RUST_BACKTRACE=1 dx serve --port 3002 --platform web --hot-reload true

