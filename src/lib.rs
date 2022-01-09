use rand::prelude::*;
use std::time::Instant;

pub struct Simulator<G, I, R>
where
    G: GenoType,
    I: Inspector<G>,
    R: Roulette<G>,
{
    population: Population<G>,
    inspector: I,
    crossover_rate: f64,
    mutation_rate: f64,
    selector: R,
    rng: ThreadRng,
    trace: Trace,
}

impl<G, I, R> Simulator<G, I, R>
where
    G: GenoType,
    I: Inspector<G>,
    R: Roulette<G>,
{
    pub fn new(
        population: Population<G>,
        inspector: I,
        crossover_rate: f64,
        mutation_rate: f64,
        selector: R,
    ) -> Self {
        Self {
            population,
            inspector,
            crossover_rate,
            mutation_rate,
            selector,
            rng: rand::thread_rng(),
            trace: Trace::default(),
        }
    }

    pub fn start(&mut self) {
        println!("started: population = {}", self.population.len());

        for i in 0.. {
            self.population = self.step_generation();

            let start = Instant::now();
            let ret = self.inspector.inspect(i, &self.population);
            let end = start.elapsed();
            self.trace.inspection.push(end.as_micros());

            if !ret {
                break;
            };
        }

        self.trace.dump();
    }

    fn step_generation(&mut self) -> Population<G> {
        let start = Instant::now();
        let selection_result = self.select_pairs();
        let end = start.elapsed();
        self.trace.selection.push(end.as_micros());

        let start = Instant::now();
        let crossover_result = self.crossover(selection_result);
        let end = start.elapsed();
        self.trace.crossover.push(end.as_micros());

        let start = Instant::now();
        let mutation_result = self.mutate(crossover_result);
        let end = start.elapsed();
        self.trace.mutation.push(end.as_micros());

        let start = Instant::now();
        let p = Population::from(mutation_result);
        let end = start.elapsed();
        self.trace.population.push(end.as_micros());

        p
    }

    fn select_pairs(&mut self) -> Vec<(G, G)> {
        let start = Instant::now();
        self.selector.reset(&self.population.inner);
        let end = start.elapsed();
        self.trace.selection_reset.push(end.as_micros());

        let mut v = vec![];

        let start = Instant::now();
        for _ in 0..self.population.inner.len() / 2 {
            let g1 = self.selector.choose();
            let g2 = self.selector.choose();
            v.push((g1, g2));
        }
        let end = start.elapsed();
        self.trace.selection_choose.push(end.as_micros());

        v
    }

    fn crossover(&mut self, mut parents: Vec<(G, G)>) -> Vec<G> {
        for (g1, g2) in parents.iter_mut() {
            let r: f64 = self.rng.gen();
            if r < self.crossover_rate {
                G::crossover(g1, g2);
            }
        }

        parents
            .into_iter()
            .map(|(g1, g2)| [g1, g2])
            .flatten()
            .collect()
    }

    fn mutate(&mut self, mut children: Vec<G>) -> Vec<G> {
        for g in children.iter_mut() {
            let r: f64 = self.rng.gen();
            if r < self.mutation_rate {
                g.mutate();
            }
        }

        children
    }
}

#[derive(Default)]
struct Trace {
    selection: Vec<u128>,
    selection_reset: Vec<u128>,
    selection_choose: Vec<u128>,
    crossover: Vec<u128>,
    mutation: Vec<u128>,
    population: Vec<u128>,
    inspection: Vec<u128>,
}

impl Trace {
    fn dump(&self) {
        let len = self.selection.len();
        println!("[dump]");
        println!("len = {}", len);
        println!(
            "selection : {:6} us, total {:6} ms",
            self.selection.iter().sum::<u128>() / len as u128,
            self.selection.iter().sum::<u128>() / 1000
        );

        println!(
            "  reset   : {:6} us, total {:6} ms",
            self.selection_reset.iter().sum::<u128>() / len as u128,
            self.selection_reset.iter().sum::<u128>() / 1000
        );

        println!(
            "  choose  : {:6} us, total {:6} ms",
            self.selection_choose.iter().sum::<u128>() / len as u128,
            self.selection_choose.iter().sum::<u128>() / 1000
        );

        println!(
            "crossover : {:6} us, total {:6} ms",
            self.crossover.iter().sum::<u128>() / len as u128,
            self.crossover.iter().sum::<u128>() / 1000
        );
        println!(
            "mutation  : {:6} us, total {:6} ms",
            self.mutation.iter().sum::<u128>() / len as u128,
            self.mutation.iter().sum::<u128>() / 1000
        );
        println!(
            "population: {:6} us, total {:6} ms",
            self.population.iter().sum::<u128>() / len as u128,
            self.population.iter().sum::<u128>() / 1000
        );
        println!(
            "inspection: {:6} us, total {:6} ms",
            self.inspection.iter().sum::<u128>() / len as u128,
            self.inspection.iter().sum::<u128>() / 1000
        );
    }
}

pub trait Inspector<G: GenoType> {
    fn inspect(&mut self, generation: usize, population: &Population<G>) -> bool;
}

pub struct Population<G: GenoType> {
    inner: Vec<(G, G::Fitness)>,
}

impl<G> Population<G>
where
    G: GenoType,
{
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn get_best(&self) -> Option<&G> {
        self.inner.iter().max_by_key(|val| val.1).map(|(g, _)| g)
    }
}

impl<G: GenoType> From<Vec<G>> for Population<G> {
    fn from(v: Vec<G>) -> Self {
        Self {
            inner: v
                .into_iter()
                .map(|g| {
                    let f = g.fitness();
                    (g, f)
                })
                .collect(),
        }
    }
}

pub trait PhenoType {
    type GenoType: GenoType;

    fn encode(&self) -> Self::GenoType;
}

pub trait GenoType: Clone {
    type Fitness: Ord + Copy;
    type PhenoType;

    fn fitness(&self) -> Self::Fitness;
    fn decode(&self) -> Self::PhenoType;
    fn mutate(&mut self);
    fn crossover(g1: &mut Self, g2: &mut Self);
}

pub trait Roulette<G: GenoType> {
    fn reset(&mut self, population: &[(G, G::Fitness)]);
    fn choose(&self) -> G;
}
