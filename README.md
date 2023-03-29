# "serde-json-core" Example

The **[serde-json-core](https://crates.io/crates/serde-json-core)** crate is a [no_std](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/using-rust-without-the-standard-library.html) variant of the popular *[serde_json](https://crates.io/crates/serde_json)* crate.

It is useful to serialize and deserialize JSON data in constrained environments (embedded devices or smart contracts e.g.).

I was not able to find any straightforward example, so here you go!

## Description

We are simulating a legit *no_std* environment on a desktop computer, by disabling all *std* features and manually re-importing the system libc (with the mere purpose of logging to the terminal with POSIX *printf()*).

The code embeds a JSON raw string, which is deserialized to a `MyJson` struct. Errors are handled almost nicely.

## Usage

> $ cargo run --release

Output:

> Successfully deserialized.
> pow.v: 9500000

If you want to see an error, you can change:

```
struct MyJson {
    mails: Vec<&'static str, 4>,
```

to:

```
struct MyJson {`
    mails: Vec<u8, 4>,`
```

("mails" is a vector of strings, not a vector of u8)

Output:

> Error: INVALID TYPE.