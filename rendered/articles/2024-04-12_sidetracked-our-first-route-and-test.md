+++
draft = true
title = "Sidetracked: Our First Route and Test"
[taxonomies]
tags = ["sidetracked", "rust"]
[extra]
toc = true
repo_link = "https://github.com/elidhu/sidetracked/tree/2024-04-12_sidetracked-our-first-route-and-test"
+++

> As the name implies, Sidetracked is finally getting some functionality, and even a test (or two!). We are going to implement a health check route and test it. With this pattern in place we can easily add more routes and tests as we go while ensuring that we are building a robust application.<!-- more -->

## Refactoring

I was planning on trying to avoid refactoring as much as possible. But I'm just a fallible human, writing these articles in parallel to Sidetracked - so I of course keep hitting things I didn't consider. Without further ado, let's refactor our crate into a library and a binary.

First thing first, let's define this by updating our `Cargo.toml`.

```toml
# ./sidetracked/Cargo.toml

[lib]
path = "src/lib.rs"
name = "sidetracked_lib"

[[bin]]
path = "src/main.rs"
name = "sidetracked"
```

A Rust crate can have as many binaries as we like, and at most one [library](https://rustc-dev-guide.rust-lang.org/backend/libs-and-metadata.html). As our snippet above shows we have defined a library and a binary. The entrypoint of the library will be located in `src/lib.rs` and the binary in `src/main.rs`. When we are done, the library will contain _almost all_ of our code except the main entrypoint for our application. This split enables us to now import the library into our binary, and more importantly into our _tests_.

Now that we have the definitions in place, we need to adjust a few things in our code to ensure we are able to compile and run again. We defined a lib at `src/lib.rs` so let's create that:

```bash
touch sidetracked/src/lib.rs
```

Resulting in the following structure:

```txt
sidetracked/src
├── lib.rs
├── main.rs
└── web
   ├── application.rs
   └── mod.rs
```

Now we need to move our application code into the library. We will start by moving the `web` module into the `lib.rs` file.

```rust
// ./sidetracked/src/lib.rs

pub mod web;
````

Once that is complete, we then need to modify the `main.rs` file to import our library.

```rust
// ./sidetracked/src/main.rs

use sidetracked_lib::web::application::Application;
```

Perfect, we are ready to proceed.

## Testing Setup

Rust has a great testing story, all of the required functionality for defining and running tests is built in. Once we have defined some tests we can simply `cargo test` to run them.

Now the first question is, where should they go?

It is quite common to see tests in the same file as the code they are testing, and we will do this for really specific tests, tests for a single function for example. However, for more complex tests, such as those that test a route, it is better to keep them separate. It enforces use of the public API, which can inform our architecture, and it saves having to debate where to put the test. It also provides an isolated place to define helper functions that can be used across multiple of these more complex tests. There is another consideration here, and that is whether you are testing private functions. If you are, then you will need to put the tests in the same file as the code, as private functions are not accessible outside of the module they are defined in.

An interesting side-note about writing tests in the `tests/` directory is that each file is compiled in to a separate crate. This can produces some unexpected results when trying to share code between tests. The solution is to define the shared code in a separate module and import it into each test file. This is what we will do with a `helpers` module. You can read more about this in the [Rust book](https://doc.rust-lang.org/book/ch11-03-test-organization.html#submodules-in-integration-tests).

```bash
files="sidetracked/tests/routes.rs sidetracked/tests/helpers/mod.rs"

for file in $files; do
  mkdir -p $(dirname $file)
  touch $file
done
```

The above snippet will create the following files:

```txt
sidetracked/tests
├── helpers
│  └── mod.rs
└── routes.rs
```

This time I will step through wiring up the modules, in the future I will probably gloss over the `use` and `mod` statements and leave it as an exercise for the reader.

So for our `routes.rs` file we will need to declare the module:

```rust
// ./sidetracked/tests/routes.rs

mod helpers;
```

Pretty simple, nothing else is required. We can verify our setup by running `cargo test`.

## Writing an Actual Test

Now that we have created the requisite files, we can take another step towards defining our first test. So to write our test, we are going to need to do two things. First we will need to start the application, presumably in such a way that it won't conflict with other tests (i.e. not on the same port). Second we will need to make a request to the health check route, before finally making some assertions about the response.

Instead of me making up some crazy strategy to achieve this, let's use a crate that has already solved this for us, [Axum test](https://crates.io/crates/axum-test).

