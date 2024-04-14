+++
draft = false
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
# @?s/cargotomllibandbin.file

@@s/cargotomllibandbin
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
// @?modweb.file

@@modweb
````

Once that is complete, we then need to modify the `main.rs` file to import our library.

```rust
// @?mainuselib.file

@@mainuselib
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
// @?modtestroutes.file

@@modtestroutes
```

Pretty simple, nothing else is required. We can verify our setup by running `cargo test`.

## Writing an Actual Test

Now that we have created the requisite files, we can take another step towards defining our first test.

What is it that we need to do in our test? Well, we ultimately want to check that we are getting the expected response from our health check route. In its current form it is just a simple route that returns a `200 OK` response when we make a `GET` request to `/health_check`.

From this description we know we need to do the following:

- Start the application
- Make a request to the health check route
- Assert that the response is a `200 OK`

We will use the Arrange/Act/Assert pattern to write our test. This pattern is a common way to structure tests, and is a good way to ensure that your tests are readable and maintainable - in my opinion at least.

Instead of me making up some crazy strategy to arrange the test, let's use a crate that has already solved this for us, [Axum test](https://crates.io/crates/axum-test).

```bash
cargo add --dev axum-test
```

We will create a small helper that will give us a test server that we can use to make requests to our application. This will be defined in the `helpers/mod.rs` file.

```rust
// @?newtestapp.file

@@newtestapp
```

I won't go in to too much detail here, it's fairly straightforward. We are creating a `TestServer` using a `TestServerConfig` and the `Router` from our `Application`. We have pre-emptively added some sensible defaults to the test server, like saving cookies, and setting the `Content-Type` to `application/json` as we are intending to build out a `JSON` API. These can all be overridden at the request level if required.

Cool, now let's use our brand new test helper in the actual test. We have already create the `routes.rs` file under the `tests` directory, so let's define the test.

```rust
// @?testhealthcheck.file

@@testhealthcheck
```

And that's really all there is to it. 3 simple steps, create a test application, make a request, and assert the response. Let's run our test and see what happens!

```txt
---- test_health_check::it_should_return_200 stdout ----
thread 'test_health_check::it_should_return_200' panicked at /Users/kglasson/.cargo/registry/src/index.crates.io-6f17d22bba15001f/axum-test-14.8.0/src/test_request.rs:589:48:
Expect status code within 2xx range, got 404 (Not Found), for request GET /health_check
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test_health_check::it_should_return_200

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

As expected our test fails, the route is not implemented yet. Let's implement the route and see if we can get our test to pass. This is about as [TDD](https://en.wikipedia.org/wiki/Test-driven_development) as I get, I promise.

### Going for Green

So we need a route that maps a `GET` at `/health_check` to a handler that returns a `200 OK`. We will define this in a `handlers` module.

```bash
touch sidetracked/src/web/handlers.rs
```

Again, directly quoting the [Axum](https://docs.rs/axum/latest/axum/index.html#handlers) documentation, a handler in Axum is:

> " In axum a “handler” is an async function that accepts zero or more “extractors” as arguments and returns something that can be converted into a response. "

This is good news, as all we need is a simple async function

```rust
// @?handlerhealth_check.file

@@handlerhealth_check
```

And let's wire up our route to use this handler. There are a few parts to the route. We can see that it has a path of `/health_check`, and that it uses an Axum helper function `get` to wrap our `health_check` handler. Axum provides a numer of these helpers, in particular there is one for each of the HTTP methods.

```rust
// @?applicationrouter.file

@@applicationrouter
```

And now, let's try running our test again.

```txt
     Running tests/routes.rs (target/debug/deps/routes-e6d43ac78857dee1)

running 1 test
test test_health_check::it_should_return_200 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

Nice, we have a passing test!

Just for fun, let's spin up the application and make a request to the health check route.

```bash
cargo run
```

And then in another terminal:

```txt
❯ curl -i http://localhost:3000/health_check
HTTP/1.1 200 OK
content-length: 0
date: Sun, 14 Apr 2024 13:43:53 GMT
```

Exactly what we expected. We are now in a good position to continue building out our application. We have a test in place that we can run to ensure that we are not breaking existing functionality, and we have a route that we can build upon.

In the next article we will look at adding some functionality to our application.

