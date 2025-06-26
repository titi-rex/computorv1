# Computor V1

A second grade polynomial solver

![meme of a surricate](https://raw.githubusercontent.com/titi-rex/computorv1/master/215559.jpg)


## What is it

This project is the first in a series that aims to make you love maths again.
The goal is to create a simple 2nd degree polynomial solver.
The language choice is free, so I used it to start learning rust.

## Build and run

```bash 
    cargo build
    cargo run -- "5 * X^0 + 4 * X^1 = 4 * X^0"
```


### Tests

```bash 
    cargo test
    ./tester.sh path/to/exe
```

This project executable is located at target/debug/computorv1.

The test script will use all `.test` file found inside `tests` dir.