use gantan::selection::{FitnessProportionate, RankSelector, TournamentSelector};
use gantan::{GenoType, Inspector, Population, SimulatorBuilder};
use std::cell::Cell;

#[derive(Clone)]
struct Gene(i32);

impl GenoType for Gene {
    type Fitness = i32;
    type PhenoType = i32;

    fn fitness(&self) -> Self::Fitness { self.0 }
    fn decode(&self) -> Self::PhenoType { self.0 }
    fn mutate(&mut self) {}
    fn crossover(_: &mut Self, _: &mut Self) {}
}

#[derive(Default)]
struct Ins { gen: Cell<usize> }

impl Inspector<Gene> for Ins {
    fn inspect(&mut self, g: usize, _p: &Population<Gene>) -> bool {
        if g != self.gen.get() {
            self.gen.set(g);
            println!("generation {} best {}", g, _p.get_best().unwrap().0);
        }
        g < 10
    }
}

fn run_with_selector<S: gantan::Roulette<Gene>>(name: &str, selector: S) {
    let pop = Population::from(vec![Gene(1), Gene(2), Gene(3)]);
    let mut builder = SimulatorBuilder::new();
    builder
        .with_population(pop)
        .with_inspector(Ins::default())
        .with_crossover_rate(0.0)
        .with_mutation_rate(0.0)
        .with_selector(selector);
    let mut sim = builder.build();
    println!("running with {}", name);
    sim.start();
}

fn main() {
    run_with_selector("fitness", FitnessProportionate::new());
    run_with_selector("tournament", TournamentSelector::new(2));
    run_with_selector("rank", RankSelector::new());
}
