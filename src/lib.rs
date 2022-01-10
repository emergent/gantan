use rand::prelude::*;
use std::collections::HashMap;
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
    stat: Stat,
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
            stat: Stat::default(),
        }
    }

    pub fn start(&mut self) {
        println!("started: population = {}", self.population.len());

        for i in 0.. {
            self.population = self.step_generation();

            if !self.inspector.inspect(i, &self.population) {
                break;
            };
        }

        self.stat.dump();
    }

    fn step_generation(&mut self) -> Population<G> {
        macro_rules! rec {
            ($tag: expr, $blk: stmt) => {{
                let start = Instant::now();
                let ret = { $blk };
                let end = start.elapsed();
                self.stat.record($tag, end.as_micros());
                ret
            }};
        }

        let selection_result = rec!("selection", self.select_pairs());
        let crossover_result = rec!("crossover", self.crossover(selection_result));
        let mutation_result = rec!("mutation", self.mutate(crossover_result));
        let p = rec!("population", Population::from(mutation_result));

        p
    }

    fn select_pairs(&mut self) -> Vec<(G, G)> {
        self.selector.reset(&self.population.inner);
        let mut v = vec![];

        for _ in 0..self.population.inner.len() / 2 {
            let g1 = self.selector.choose();
            let g2 = self.selector.choose();
            v.push((g1, g2));
        }

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
struct Stat {
    inner: HashMap<String, Vec<u128>>,
}

impl Stat {
    fn record(&mut self, tag: &str, value: u128) {
        let v = self.inner.entry(tag.to_string()).or_insert_with(Vec::new);
        v.push(value);
    }

    fn dump(&self) {
        println!("[dump]");
        for (k, v) in &self.inner {
            let len = v.len() as u128;
            let sum = v.into_iter().sum::<u128>();
            if len > 0 {
                println!(
                    "{}\t: average {:6} us,\ttotal {:6} ms",
                    k,
                    sum / len,
                    sum / 1000
                );
            }
        }
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
