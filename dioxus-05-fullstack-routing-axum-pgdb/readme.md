## Dioxus 0.5 Fullstack with Routing, Axum, and Postgres

Just another sample project that showcases the usage of:

-   Dioxus Fullstack
-   Including routing to pages
-   And using server functions that talk with Postgres

---

What does work:

-   The routing (both navigation and url path)
-   The SSR (only SSR, not CSR)

What doesn't work:

-   "Get Server Data" link
    -   On click, no call to the back-end is triggered<br/>
        (and so the corresponding server function is not called)
