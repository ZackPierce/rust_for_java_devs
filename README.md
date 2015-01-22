# rust_for_java_devs

An insanely over-documented Rust library that introduces
Rust by tackling a simple technical design challenge and drawing a
lot of direct analogies to Java.

## Technical Challenge

A supermarket sells 3 products listed below:

* Product A = $20
* Product B = $50 (or 5 for the price of 3)
* Product C = $30

Implement the code for a checkout reigster that calculates the price of a given sequence of items. The input is a product list as a String, e.g. "ABBACBBAB" : for which the output should be the integer 240. Please consider testability, documentation and good coding practices in your solution. As an additional challenge, consider how new pricing rules might be provided programmatically.

Implement the following:

```
    public class Supermarket {
        public int checkout(String items);
    }
```

This exercise originated with a trendy startup company that will remain
anonymous to minimize impact to their hiring flow.

## Installation

First, install the Rust Nightly by following the directions at http://www.rust-lang.org/install.html

```
    git clone https://github.com/ZackPierce/rust_for_java_devs.git
    cd rust_for_java_devs
    cargo build
```

## Learn

I suggest starting your reading in the `src/lib.rs` file, from top to bottom,
and then move on to `tests/lib.rs`

If you need a reference to the underlying approach, look in the `java` directory
for a parallel Java implementation.

If you're curious about the packaging mechanism, "cargo", look at `Cargo.toml`.
Because this library currently has no external dependencies, there's not a lot
to see there at the moment.

## Prove that it works

Run the tests via:

```
    cargo test
```

Or, if you want to check the java version, run:

```
   mvn clean test
```

