## Issue 2343 Reproducible Example

This example reproduces the issue [2343](https://github.com/DioxusLabs/dioxus/issues/2343) that exists in Dioxus 0.5.1.

### Usage

1. In a terminal, run `./run_css.sh` to compile the CSS.
2. In another terminal, run `./run_dev` to run the example.

### The Issue

To reproduce the issue:

-   click on 2nd tab
-   click on 1st tab
-   click on 2nd tab
-   click on 1st tab.

The error thown to the _Console_ looks like this:

```
panicked at /home/dxps/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dioxus-core-0.5.1/src/arena.rs:57:32:
cannot reclaim ElementId(12)

Stack:

__wbg_get_imports/imports.wbg.__wbg_new_abda76e883ba8a5f/<@http://localhost:8080/assets/dioxus/dioxus-issue-2343.js:759:21
logError@http://localhost:8080/assets/dioxus/dioxus-issue-2343.js:256:18
__wbg_get_imports/imports.wbg.__wbg_new_abda76e883ba8a5f@http://localhost:8080/assets/dioxus/dioxus-issue-2343.js:758:66
dioxus_issue_2343-7a3869c67d5118c4.wasm.console_error_panic_hook::Error::new::haaa4b7741c58d502@http://localhost:8080/assets/dioxus/dioxus-issue-2343_bg.wasm:wasm-function[8873]:0x33a4dd
dioxus_issue_2343-7a3869c67d5118c4.wasm.console_error_panic_hook::hook_impl::hbe4f4c374010cc7a@http://localhost:8080/assets/dioxus/dioxus-issue-2343_bg.wasm:wasm-function[1942]:0x209c6f
dioxus_issue_2343-7a3869c67d5118c4.wasm.console_error_panic_hook::hook::h90c70b0f91772647@http://localhost:8080/assets/dioxus/dioxus-issue-2343_bg.wasm:wasm-f…
dioxus-issue-2343.js:753:21
Uncaught RuntimeError: unreachable executed
dioxus-issue-2343_bg.wasm:3625130:1
panicked at /home/dxps/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasm-bindgen-futures-0.4.42/src/task/singlethread.rs:102:37:
already borrowed: BorrowMutError

Stack:

__wbg_get_imports/imports.wbg.__wbg_new_abda76e883ba8a5f/<@http://localhost:8080/assets/dioxus/dioxus-issue-2343.js:759:21
logError@http://localhost:8080/assets/dioxus/dioxus-issue-2343.js:256:18
__wbg_get_imports/imports.wbg.__wbg_new_abda76e883ba8a5f@http://localhost:8080/assets/dioxus/dioxus-issue-2343.js:758:66
dioxus_issue_2343-7a3869c67d5118c4.wasm.console_error_panic_hook::Error::new::haaa4b7741c58d502@http://localhost:8080/assets/dioxus/dioxus-issue-2343_bg.wasm:wasm-function[8873]:0x33a4dd
dioxus_issue_2343-7a3869c67d5118c4.wasm.console_error_panic_hook::hook_impl::hbe4f4c374010cc7a@http://localhost:8080/assets/dioxus/dioxus-issue-2343_bg.wasm:wasm-function[1942]:0x209c6f
dioxus_issue_2343-7a3869c67d5118c4.wasm.console_error_panic_hook::hook::h90c70b0f91772647@http://localhost:8080/assets/dioxus/dioxu…
dioxus-issue-2343.js:753:21
Uncaught RuntimeError: unreachable executed
```
