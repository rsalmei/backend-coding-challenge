# Bipa Back-end Coding Challenge

Hello! Thank you for taking the time to review my coding challenge submission for Bipa.

It was a fun and interesting project that allowed me to explore new technologies and, hopefully, demonstrate my skills.

Since it was very simple, although fairly reasonable for the time given (and being a take-home challenge, of course), I seized the opportunity to experiment with some technologies I had not used before. One is SurrealDB, and another, believe it or not, is Axum!

SurrealDB is overkill, of course. Even when embedded, it provides a multi-model database with relational, document, graph, time-series, vector, geospatial, and full-text search! I could have used something much simpler like SQLite or the amazing PostgreSQL, but I wanted to see how SurrealDB worked and how easy it was to integrate it into a Rust back-end. And considering it could eventually scale from embedded to a full-fledged distributed cluster without changing a single line of code, it seemed great to try it out.
<br>For this project, I used its embedded mode with in-memory storage, which is perfect for testing and small applications. But you can easily switch to a persistent storage engine or even a distributed setup if wanted.

Curiously, all the companies I've worked for so far have used Warp for Rust back-ends, so I thought it was time to try Axum myself, which I only hear good things about. I read a lot about it throughout the years, and I really like its design and philosophy. For instance, I love the magic handlers and extractors, and how ergonomic it seems to be, so I'll finally get to use it in a small project.

## Build tools & versions used

I always update Rust to the latest stable version the day it comes out, so at the time of writing, I used Rust 1.92.0.

The dependencies are all the latest versions available on crates.io, and I tend to declare them using the shortest possible version specifier (e.g., `2.4` instead of `2.4.1`, or even just `1` when feasible), so that `cargo` can resolve to the latest compatible versions when building the project. And I always use `cargo update` to bring all dependencies to their latest versions whenever I finish a PR.

My code is always formatted using `rustfmt` with default settings, and I always use `clippy` to lint the code and make sure it is idiomatic and follows best practices.

## Steps to run the app

1. Clone the repository and cd into the project folder
2. Cargo run the server:
   ```bash
   cargo run --release
   ```
3. There's no step 3! The server will be running at `http://localhost:3123`

If you do want to switch the database from in-memory to persistent, just include the `kv-surrealkv` feature in the `surrealdb` dependency in `Cargo.toml`, and change the `SURREALDB` environment variable to point to a file path:

```toml
surrealdb = { version = "2.4", default-features = false, features = ["kv-mem", "kv-surrealkv"] }
```
Then run the server with:
```bash
SURREALDB="surrealkv://bipa-challenge.db" cargo run
```

## What was the reason for your focus? What problems were you trying to solve?

I wanted to create the simplest possible back-end that met the requirements, but without ever leaving quality behind. I wanted to demonstrate my ability to write clean, maintainable, modular, and efficient code like I would in a real-world project.

For example, I didn't need a generic periodic task scheduler for this project as it has only one background task, but I implemented it anyway just to show how I would approach such a concern in a real-world scenario. I'm sure a project like this would consume more data over time, perhaps from other providers, and perform other tasks in the background, thus having a robust and flexible scheduler would for sure be beneficial in the long run. I also made sure to handle errors gracefully, and write clear comments throughout the codebase where doubts could arise.

Finally, I wanted to ensure that the data fetched from the external API was always consistent and up-to-date. For that, I implemented the storage handler with an UPSERT mechanism that atomically updates existing records or inserts new ones as needed. Also, I wanted to make the background data fetching routine was robust and reliable, never breaking even in the face of network errors or unexpected API responses. I also wanted to make sure that the code was easy to read and understand, so that other developers could easily pick it up and work on it in the future.

## How long did you spend on this project?

I spent around 8 hours on this project, spread over the weekend. I'm sure I could have done it in two or three hours top if I had focused solely on known technologies, but I wanted to take my time to explore new tools and have fun with it. It was worth it!

## Did you make any trade-offs for this project? What would you have done differently with more time?

Yes, I made some trade-offs to keep the project simple and focused on the core requirements. For instance, I opted for very simple println! logging instead of using a full-fledged logging framework like `tracing` and `subscriber`, to avoid adding unnecessary complexity to the codebase. I also decided to use environment variables for configuration instead of a more sophisticated argument parser or configuration file, as the configuration needs were minimal for this project. And better errors for the internal API, with proper status codes and error messages, would be nice.

Another debatable trade-off I made was to store the fetched data raw as received from the external API, without any transformation or normalization. This made the storage handler simpler, more efficient and reliable, but it also means I had to do some data processing on the internal API, which required allocating memory and converting the data on each incoming request. Although this seems inefficient, I believe I would approach it in a real-world project exactly the same, as it keeps the data as close to the source as possible, and allows for more flexibility afterward by decoupling the storage format from the API format.

With more time, I would have included logging for internal API requests, colorized terminal output for better readability, and perhaps some integration tests.

On a real-world project, I would also consider adding:
- observability with metrics and tracing,
- a health check endpoint,
- rate limiting for the external API requests,
- authentication and authorization for the internal API,
- pagination for the internal API responses,
- etc.

## What do you think is the weakest part of your project?

I believe the current implementation is solid and meets the requirements well, but if I had to point out a weak spot, it would be the lack of proper logging.

## Is there any other information you’d like us to know?

Just a style guide I try to follow: please note I write code comments like I would in a production codebase in a real-world project. That means I try to write them like the code isn't mine but the team's, so I avoid personal pronouns like "I" in preference to "we" or neutral, using the imperative mood. That makes it clearer that the code is a collective effort of the team, not just one person's work, and I believe it fosters better collaboration and ownership among team members.
<br>Here in this project, however, I did use "I" at times, to better express my personal decisions and thoughts behind certain implementations.
