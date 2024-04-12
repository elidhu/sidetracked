+++
draft = true
title = "Sidetracked: Introducing the Project and Getting Setup"
[taxonomies]
tags = ["sidetracked", "rust"]
[extra]
toc = true
repo_link = "https://github.com/elidhu/sidetracked/tree/2024-04-10-sidetracked-introducing-the-project-and-getting-setup"
+++

> Sidetracked is going to be, yep you guessed it, a todo application. We are going to write it in Rust (of course) and we are going to draw _loosely_ from [Domain-driven Design](https://en.wikipedia.org/wiki/Domain-driven_design) and [Hexagonal Architecture](https://en.wikipedia.org/wiki/Hexagonal_architecture_(software)) for our project architecture. This will be a fully featured application, including a Backend using [Axum](https://docs.rs/axum/latest/axum/), and a Frontend using [Htmx](https://htmx.org/).<!-- more -->

## A Very Loose Roadmap / Feature List

Even I don't know exactly how this is going to turn out, so lets make a list of what features we really want to explore (via implementing them). I find that some tutorial style articles usually gloss over the "hard" stuff, so let's _try_ not to do that.

- Backend - Axum, Utoipa (for OpenAPI schema), JWT Authentication, Sqlx
- Frontend - Htmx, DaisyUI, ChatGPT (Design, Colors, Logo)
- CLI - Clap

## Getting Set Up

Alright, let's get set up. I'm going to start with a [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). This may? not be necessary as I'm sure a single crate application would be sufficient, but I always find that I, at some point, want to split out another crate. So even if we only end up with a single crate, no harm done.

First set up a directory and an empty `Cargo.toml` for our workspace. At the time of writing there is no `cargo` command to initialise a workspace, but I'm sure their will be soon (I really just expected it to work).

```bash
mkdir sidetracked && cd sidetracked
touch Cargo.toml
```

Then fill in the contents of the `Cargo.toml`. This indicates we will have a crate at `./sidetracked`.

```toml
# ./Cargo.toml

[workspace]
resolver = "2"
members = ["sidetracked"]
```

Now create the crate. We will start with a simple `bin` crate for now, we will have an internal `lib` but probably break it out into a separate crate later on.

```bash
cargo init --bin sidetracked
```

Lets check everything is working by running the Hello World `bin` that `cargo init` generates.

```txt
❯ cargo run
   Compiling sidetracked v0.1.0 (/Users/kglasson/S/github.com/elidhu/sidetracked/sidetracked)
    Finished dev [unoptimized + debuginfo] target(s) in 1.02s
     Running `target/debug/sidetracked`
Hello, world!
```

Easy! If this is not working then you will need to retrace your steps and figure out what's wrong!

## A Minimal Web Server

First thing first, let's get a minimal web server going, for that we are going to use Axum.

Why Axum? Well, I like it.

It is also quite popular within the ecosystem, meaning it is easier to find help with, uses other another popular crate ([Tower](https://github.com/tower-rs/tower)) to provide it's middleware, and it is maintained by the [Tokio](https://github.com/tokio-rs) GitHub organisation. Tokio provides a-lot of very useful crates in the Rust ecosystem. But I digress, let's get a server up.

Make your `Cargo.toml` look like the following. To be clear, this is the `Cargo.toml` for the `sidetracked` crate, _not_ the `workspace`.

```toml
# ./sidetracked/Cargo.toml

[package]
name = "sidetracked"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
anyhow = "1.0.82"
axum = { version = "0.7.5", features = ["macros"] }
chrono = "0.4.37"
serde = { version = "1.0.197", features = ["derive"] }
tokio = { version = "1.37.0", features = ["full"] }
tower = "0.4.13"
tower-http = { version = "0.5.2", features = ["trace"] }
tracing = "0.1.40"
tracing-subscriber = { version = "0.3.18", features = [
  "env-filter",
  "json",
  "chrono",
] }
```

> As an artifact of how I stitch together my articles, the code snippets will be in their "end-of-article" form. For example in the above, we don't _need_ to add all of the dependencies at the start, we could do it progressively as we pull them in to the project.

Now let's set up a few directories, I did say minimal, but I don't mean want to do any refactoring _during_ the article, we'll save that for future articles if necessary.

```bash
mkdir -p sidetracked/src/web
touch sidetracked/src/web/{mod,application}.rs
```

And add the following snippets to the respective files to create two modules. The `web` module, which will ultimately contain our server code, the code that interacts with the outside world. Followed by the `application` module which is going to house the bulk of the router setup. If you need a refresher on how Rust organises modules, take a look this section in the [Rust book](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html).

```rust
// ./sidetracked/src/main.rs

pub mod web;
```
<br>

```rust
// ./sidetracked/src/web/mod.rs

pub mod application;
```

### An Application Struct

Now let's start populating these files with some actual code. A pretty common pattern that I see in the wild that I like is to have an `Application` `struct`. This is where we hold the configuration for the server, the methods to compose the routes, and the method to run the server.

```rust
// ./sidetracked/src/web/application.rs

pub struct Application {
    /// The host to listen on
    pub host: IpAddr,
    /// The port to listen on
    pub port: u16,
}
```

The above is pretty self-explanatory so lets get on with some implementation. In Rust, to attach methods to a `struct`, you need to use an `impl` block. This is where we will put the `new` method, which will create a new `Application` `struct`, the `router` method that will compose our routes, and the `run` method, which will start the server.

```rust
// ./sidetracked/src/web/application.rs

impl Application {
    /// Create a new application
    pub fn new(host: IpAddr, port: u16) -> Self {
        Self { host, port }
    }

    /// Create the application router
    pub async fn router(&self) -> Router {
        Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
    }

    /// Run the application
    pub async fn run(&self) {
        let addr = SocketAddr::new(self.host, self.port);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        let router = self.router().await;

        info!("Listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, router.into_make_service())
            .await
            .expect("Unexpected error during server execution");
    }
}
```

> Some things to not about the above, take note that the `TcpListener` we are using is from the `tokio::net` module, not the standard library. This is because we are using Tokio's runtime to run our server. I have spent far too much time debugging this particular thing when I was first trying to learn Rust, so I thought I would point it out.

And finally, just one little bit of sugar to make our lives easy in the beginning. We are doing to implement the `Default` [trait](https://doc.rust-lang.org/book/ch10-02-traits.html) for our `Application` `struct`, so that we can easily create a new `Application` with some sensible defaults.

```rust
// ./sidetracked/src/web/application.rs

impl Default for Application {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".parse().unwrap(),
            port: 3000,
        }
    }
}
```

So what have we created?

We now have a minimal web server, it has a single route at `/` that will respond with `Hello, World!` when it receives a `GET` request, we have a `run` method that will start the server, and we have an `Application` `struct` to encapsulate it all.

Digging in a little further, we can see that we are adding a `Layer` to our `Router`. This is how Axum does middleware, by leaning on the Tower ecosystem. What we have here is `TraceLayer` from the `tower_http` crate. This is a middleware that will log all requests that come into our server. I consider this to be essential, especially during development, so that we can see what is going on. Maybe in production you log all requests at a load balancer or something, but for now, this is fine.

I highly recommend reading the [Axum middleware documentation on ordering](https://docs.rs/axum/latest/axum/middleware/index.html#ordering) as it is very important to understand how the layers are applied, and in what order. This is a common source of confusion in my experience, as the order of middleware application differs between calling `.layer()` on the `Router` and calling `.layer()` on the `ServiceBuilder`. Quoting the documentation:

> " It’s recommended to use tower::ServiceBuilder to apply multiple middleware at once, instead of calling layer (or route_layer) repeatedly "

This is because the "top-to-bottom" ordering is often more natural.

We aren't actually "running" the server yet, there is one more thing we should set up first.

### Tracing and Logging

If we were to actually run our server, we wouldn't see anything output to the console. While we have used the `TraceLayer` middleware, and the `info!` macro, we haven't configured the logger to actually output anything. So let's do that now.

I have this snippet that I use in most of my Axum projects, it sets up the logger to either accept the `RUST_LOG` environment variable, or fall back to some sensible defaults. Those sensible defaults include DEBUG level logging for the current crate, DEBUG logging for `tower_http` and TRACE logging for `axum::rejection` (we will get to what this even is in the future). This snippet is basically straight from the [Axum documentation](https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging/src/main.rs) with only a tiny tweak so that I can copy-paste it without any changes.

```rust
// ./sidetracked/src/main.rs

fn init_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                let crate_name = env!("CARGO_CRATE_NAME");
                format!("{crate_name}=debug,tower_http=debug,axum::rejection=trace").into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
```

A pretty standard pattern for logging that is also adhered to in Rust is to split it in to two parts. The part that emits the events, in this case `tracing`, and the part that handles them, in this case `tracing-subscriber`. This is a pretty powerful pattern, as it allows you to change the way you handle logs without changing the way you emit them. For example, you could log to a file, or a network socket, or a log aggregation service or all three, and all you would need to do is reconfigure the initialisation logic.

### Running the Server

With that out of the way, let's actually run the server.

```rust
// ./sidetracked/src/main.rs

#[tokio::main]
async fn main() {
    init_logging();

    let app = Application::default();
    app.run().await;
}
```

Let's take it for a spin. I like to utilise `cargo watch` for this, as it will automatically recompile the code when it changes. If you don't have it, you can install it with `cargo install cargo-watch`.

```bash
cargo watch -x run -w sidetracked
```

You should see some output in your terminal indicating that the server is listening on `127.0.0.1:3000`. We can now test it by running `curl http://localhost:3000` in another terminal. You should see `Hello, World!` returned. Something like this:

```shell
❯ curl http://localhost:3000
Hello, World!
```

Now from the terminal that is running the server, you should see some output from the logger. Something like this:

```shell
❯ cargo watch -x run -w sidetracked
[Running 'cargo run']
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/sidetracked`
2024-04-11T05:55:42.213495Z  INFO sidetracked::web::application: Listening on 127.0.0.1:3000
2024-04-11T05:57:28.517081Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-04-11T05:57:28.517193Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=0 ms status=200
```

## Next steps

I think this is probably a logical stopping point for this article, otherwise it could, quite literally go on forever.

