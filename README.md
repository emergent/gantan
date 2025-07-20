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

### Building a simulator

`SimulatorBuilder` provides a convenient chained API to configure a simulator:

```rust
let mut simulator = SimulatorBuilder::new()
    .with_population(population)
    .with_inspector(inspector)
    .with_crossover_rate(0.9)
    .with_mutation_rate(0.05)
    .with_selector(selector)
    .with_seed(42) // optional
    .build();
```

