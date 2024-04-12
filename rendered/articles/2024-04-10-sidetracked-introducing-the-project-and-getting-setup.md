+++
draft = true
title = "Sidetracked: Introducing the Project and Getting Setup"
[taxonomies]
tags = ["sidetracked", "rust"]
[extra]
toc = true
+++

> Sidetracked is going to be, yep you guessed it, a todo application. We are going to right it in Rust (of course). We are going to draw _loosely_ from [Domain-driven Design](https://en.wikipedia.org/wiki/Domain-driven_design) and [Hexagonal Architecture](https://en.wikipedia.org/wiki/Hexagonal_architecture_(software)) for our project architecture. This will be a fully features application, we are going to write our Backend using [Axum](https://docs.rs/axum/latest/axum/), and the Frontend will be [Htmx](https://htmx.org/).<!--more-->

## A Very Loose Roadmap / Feature List

Even I don't know exactly how this is going to turn out, so lets make a list of what features we really want to explore (via implementing them). I find that some tutorial style articles usually gloss over the "hard" stuff, so let's _try_ not to do that.

- Backend API - Axum, Utoipa (for OpenAPI schema), JWT Authentication, Sqlx
- CLI - Clap
- Frontend - Htmx, DaisyUI, ChatGPT (Design, Colors, Logo)

## Getting set up

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

```bash
❯ cargo run
   Compiling sidetracked v0.1.0 (/Users/kglasson/S/github.com/elidhu/sidetracked/sidetracked)
    Finished dev [unoptimized + debuginfo] target(s) in 1.02s
     Running `target/debug/sidetracked`
Hello, world!
```


