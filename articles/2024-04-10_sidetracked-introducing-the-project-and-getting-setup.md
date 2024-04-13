+++
draft = false
title = "Sidetracked: Introducing the Project and Getting Setup"
[taxonomies]
tags = ["sidetracked", "rust"]
[extra]
toc = true
repo_link = "https://github.com/elidhu/sidetracked/tree/2024-04-10_sidetracked-introducing-the-project-and-getting-setup"
+++

> Sidetracked is, yep you guessed it, a todo application. We are going to write it in Rust (of course) and we are going to draw _loosely_ from [Domain-driven Design](https://en.wikipedia.org/wiki/Domain-driven_design) and [Hexagonal Architecture](https://en.wikipedia.org/wiki/Hexagonal_architecture_(software)) for our project architecture. This will be a fully featured application, including a Backend using [Axum](https://docs.rs/axum/latest/axum/), and a Frontend using [Htmx](https://htmx.org/).<!-- more -->

## A Very Loose Roadmap / Feature List

Even I don't know exactly how this is going to turn out, so lets make a list of what technologies and features we really want to explore (via implementing them). I find that some tutorial style articles usually gloss over the "hard" stuff, so let's _try_ not to do that.

### Technologies

- Backend
  - Axum: Web server
  - Utoipa: To generate OpenAPI documentation
  - Authentication and Authorization using JWTs
  - Persistence: Postgres and Sqlx
- Frontend
  - Htmx: No/low javascript framework
  - DaisyUI (and tailwind): Styling
  - ChatGPT: (Design, Colors, Logo)
- CLI
  - Clap: Command line parsing

### Features

- Todos (obviously)
- Users
- Sharing
- Reminders and notifications


## Getting Set Up

Alright, let's get set up. I'm going to start with a [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). This may not be necessary as I'm sure a single crate application would be sufficient, but I always find that I, at some point, want to split out another crate. So even if we only end up with a single crate, no harm done. As an aside, I lean towards the enforced isolation that crates provide, especially when I'm trying to figure out the API of something I _know_ I want to keep separate. It stops me just reaching in and changing things, and forces me to plan a little more. This is obviously not required, and is just a personal preference.

First, set up a directory and an empty `Cargo.toml` for our workspace. At the time of writing there is no `cargo` command to initialise a workspace, but I'm sure their will be soon (I really just expected it to work).

```bash
mkdir sidetracked && cd sidetracked
touch Cargo.toml
```

Then fill in the contents of the `Cargo.toml`. This indicates we will have a crate at `./sidetracked`.

```toml
# @?cargotomlworkspace.file

@@cargotomlworkspace
```

Now to actually initialise the crate with `cargo`. This will create a minimal Rust binary crate that is able to be compiled and run.

```bash
cargo init --bin sidetracked
```

Lets check everything is working by running the binary.

```txt
❯ cargo run
   Compiling sidetracked v0.1.0 (/Users/kglasson/S/github.com/elidhu/sidetracked/sidetracked)
    Finished dev [unoptimized + debuginfo] target(s) in 1.02s
     Running `target/debug/sidetracked`
Hello, world!
```

Easy! If this is not working then you will need to retrace your steps and figure out what's wrong!

## A Minimal Web Server

Okay, so we have _something_ working. Let's expand on our generated "Hello, world!" and get a minimal web server going, for that we are going to use Axum.

Why Axum? Well, I like it.

It is also quite popular within the ecosystem, meaning it is easier to find help with, uses another popular crate ([Tower](https://github.com/tower-rs/tower)) to provide it's middleware, and it is maintained by the [Tokio](https://github.com/tokio-rs) GitHub organisation. Tokio provides a-lot of very useful crates in the Rust ecosystem. But I digress, let's get a server up.

Make your `Cargo.toml` look like the following. To be clear, this is the `Cargo.toml` for the `sidetracked` crate, _not_ the `workspace`.

```toml
# @?cargotomlsidetracked.file

@@cargotomlsidetracked
```

> As an artifact of how I stitch together my articles, the code snippets will be in their "end-of-article" form. For example, in the above snippet we have all of the dependencies I have added, not necessarily the ones I added in this step.

Now let's set up a few directories, I did say minimal, but I don't want to do too much refactoring _during_ the article, we'll save that for future articles if necessary.

```bash
mkdir -p sidetracked/src/web
touch sidetracked/src/web/{mod,application}.rs
```

Add the following snippets to their respective files to create two modules. The `web` module, which will ultimately contain our server code, the code that interacts with the outside world. Followed by the `application` module which is going to house the bulk of the router setup. If you need a refresher on how Rust organises modules, take a look at the following section in the [Rust book](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html).

```rust
// @?modweb.file

@@modweb
```
<br>

```rust
// @?modapplication.file

@@modapplication
```

### An Application

Now let's start populating these files with some actual code. A pretty common pattern that I see in the wild that I like is to have an `Application` struct. This will encapsulate all of the composing of routes and middleware, plus any relevant helpers that define the server. This is a good way to keep the `main` function clean and concise.

```rust
// @?structapplication.file

@@structapplication
```

The above is pretty self-explanatory as currently our `Application` doesn't require any data, so lets get on with the implementation. In Rust, to attach methods to a struct, you need to use an `impl` block where we will define the `router` method that constructs the `Router` for our server.

```rust
// @?implapplication.file

@@implapplication
```

We will also define an `ApplicationConfig` struct to hold some values that we might want to configure in the future.

```rust
// @?structapplicationconfig.file

@@structapplicationconfig
```

Finally, just one little bit of sugar to make our lives easier. We are going to implement the `Default` [trait](https://doc.rust-lang.org/book/ch10-02-traits.html) for our `ApplicationConfig` struct. This will enable us to easily create a new `ApplicationConfig` with some sensible defaults.

```rust
// @?impldefaultapplicationconfig.file

@@impldefaultapplicationconfig
```

And let's also define a run function to actually run the server using our `Application` and `ApplicationConfig`.

```rust
// @?run.file

@@run
```

> A note about the above. The `TcpListener` we are using is from the `tokio::net` module, not the standard library. This is because we are using Tokio's runtime to run our async code. I have spent far too much time debugging this particular thing when I was first trying to learn Rust, so I thought I would point it out. Tokio does it's best to mimic the stdlib structure, and autocomplete sometimes can lead you astray.

So what have we created?

We now have a minimal web server, it has a single route at `/` that will respond with `Hello, World!` when it receives a `GET` request, we have a `run` method that will start the server, and we have an `Application` struct to define our routes, and an `ApplicationConfig` to hold our configuration.

Let's take a look at our `Router`.

```rust
// @?applicationrouter.file

@@applicationrouter
```

We can see that we are adding a `Layer` to our `Router`. This is how Axum does middleware, by leaning on the Tower ecosystem. What we have here is `TraceLayer` from the `tower_http` crate. This is a middleware that will log all requests that come into our server. I consider this to be essential, especially during development, so that we can see what is going on!

I highly recommend reading the [Axum middleware documentation on ordering](https://docs.rs/axum/latest/axum/middleware/index.html#ordering) as it is very important to understand how the layers are applied, and in what order. This is a common source of confusion in my experience, as the order of middleware execution differs between calling `.layer()` on the `Router` and calling `.layer()` on the `ServiceBuilder`. Quoting the documentation:

> " It’s recommended to use tower::ServiceBuilder to apply multiple middleware at once, instead of calling layer (or route_layer) repeatedly "

This is because the "top-to-bottom" ordering provided by `ServiceBuilder` is often more natural.

Now, we aren't actually _running_ the server yet, there is one more thing we should set up first.

### Tracing and Logging

If we were to actually run our server, we wouldn't see anything output to the console. While we have used the `TraceLayer` middleware, and the `info!` macro, we haven't configured the logger to actually output anything where we can see it. So let's do that now.

I use the following snippet in most of my Axum projects. It sets up the logger to either accept the `RUST_LOG` environment variable, or fall back to some sensible defaults. Those sensible defaults include `DEBUG` level logging for the current crate, `DEBUG` logging for `tower_http` and `TRACE` logging for `axum::rejection` (we will get to what this even is in the future). This snippet is straight from the [Axum examples](https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging/src/main.rs) with only a tiny tweak so that I can copy-paste it without any changes.

```rust
// @?initlogging.file

@@initlogging
```

A pretty standard pattern for logging that is also adhered to in Rust is to split it in to two parts. The part that emits the events, in this case that is `tracing`, and the part that handles the events, in this case `tracing-subscriber`. This is a pretty powerful pattern, as it allows you to change the way you handle logs without changing the way you emit them. For example, you could log to a file, a network socket, a log aggregation service, or all three. The only thing you would need to do is reconfigure the initialisation logic.

### Running the Server

With that out of the way, let's configure the server.

```rust
// @?main.file

@@main
```

> For those of you new to Rust (or maybe just new to async Rust), the `tokio::main` macro is a helper macro that sets up the Tokio runtime for you. You can read more about how that works [here](https://docs.rs/tokio-macros/latest/tokio_macros/attr.main.html).

The code is pretty straightforward. We initiliase the logging before anything else. We then set up our `ApplicationConfig` using our magical `Default` implementation, we crate the `Application` and then we run the server!

Let's take it for a spin. I like to utilise `cargo watch` for this, as it will automatically recompile the code when it changes. Most developer workflows will use something like this to "hot reload" changes. If you don't already have `cargo watch`, you can install it with `cargo install cargo-watch`.

```bash
cargo watch -x run -w sidetracked
```

You should now see some output in your terminal indicating that the server is listening on `127.0.0.1:3000`. We test our brand new server by hitting it with `curl -i http://localhost:3000` from another terminal. You should see something like this:

```txt
❯ curl -i http://localhost:3000
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 13
date: Thu, 11 Apr 2024 13:38:07 GMT

Hello, World!
```

Perfect, as expected we get a `200` status code and a `Hello World` in the body. Now from the terminal that is running the server you should see output from the logger. Something like this:

```txt
❯ cargo watch -x run -w sidetracked
[Running 'cargo run']
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/sidetracked`
2024-04-11T05:55:42.213495Z  INFO sidetracked::web::application: Listening on 127.0.0.1:3000
2024-04-11T05:57:28.517081Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-04-11T05:57:28.517193Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=0 ms status=200
```

## Next steps

I think this is probably a logical stopping point otherwise it could, quite literally, go on forever. We have set up a basic project and are ready to start building out some more features. In the next article we will start building out the API.

