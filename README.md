# gantan

`gantan` is a lightweight framework for building Genetic Algorithm (GA) based solutions in Rust. It provides abstractions for genotypes, phenotypes, selection strategies and a simulator to evolve populations.

This crate focuses on providing just the building blocks required to implement a genetic algorithm. You define your own gene representation, how it mutates and how individuals are evaluated. `gantan` then takes care of evolving a population using crossover, mutation and selection.


## Running the example

A sample Traveling Salesman Problem solver is included.

1. Clone the repository
2. Run the built-in example:

```shell
cargo run --release --example tsp
```


The example prints the best distance found every few generations.

## Using as a library

Add `gantan` to your `Cargo.toml` and implement the required traits (`GenoType`, `PhenoType`, `Inspector` and `Roulette`) for your problem domain.
### Defining your types

Implement the following traits for your domain:

```rust
#[derive(Clone)]
struct MyGene { /* fields */ }

struct MyPhenotype;

impl GenoType for MyGene {
    type Fitness = u32;
    type PhenoType = MyPhenotype;

    fn fitness(&self) -> Self::Fitness { /* ... */ }
    fn decode(&self) -> Self::PhenoType { /* ... */ }
    fn mutate(&mut self) { /* ... */ }
    fn crossover(g1: &mut Self, g2: &mut Self) { /* ... */ }
}

impl PhenoType for MyPhenotype {
    type GenoType = MyGene;

    fn encode(&self) -> MyGene { /* ... */ }
}

struct MyInspector;

impl Inspector<MyGene> for MyInspector {
    // The `inspect` method is called at the end of each generation. Returning `true` continues
    // the simulation, while returning `false` stops it. Here, we stop the simulation after 100 generations.
    fn inspect(&mut self, generation: usize, _pop: &Population<MyGene>) -> bool {
        generation < 100
    }
}
```


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

After configuring the builder you obtain a `Simulator`. The `start` method begins the evolution loop:

```rust
simulator.start();
```

