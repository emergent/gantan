# gantan

`gantan` is a lightweight framework for building Genetic Algorithm (GA) based solutions in Rust. It provides abstractions for genotypes, phenotypes, selection strategies and a simulator to evolve populations.

## Running the example

A sample Traveling Salesman Problem solver is included. You can run it with:

```shell
cargo run --release --example tsp
```

The example prints the best distance found every few generations.

## Using as a library

Add `gantan` to your `Cargo.toml` and implement the required traits (`GenoType`, `PhenoType`, `Inspector` and `Roulette`) for your problem domain.

