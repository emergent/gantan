use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::cell::RefCell;
use crate::{GenoType, Roulette};

/// Fitness proportionate selection (roulette wheel)
pub struct FitnessProportionate<G: GenoType>
where
    G::Fitness: Into<f64> + Copy,
{
    inner: Vec<(G, f64)>,
    sum: f64,
    rng: RefCell<StdRng>,
}

impl<G> FitnessProportionate<G>
where
    G: GenoType,
    G::Fitness: Into<f64> + Copy,
{
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            sum: 0.0,
            rng: RefCell::new(StdRng::from_entropy()),
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            inner: Vec::new(),
            sum: 0.0,
            rng: RefCell::new(StdRng::seed_from_u64(seed)),
        }
    }
}

impl<G> Default for FitnessProportionate<G>
where
    G: GenoType,
    G::Fitness: Into<f64> + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<G> Roulette<G> for FitnessProportionate<G>
where
    G: GenoType,
    G::Fitness: Into<f64> + Copy,
{
    fn reset(&mut self, population: &[(G, G::Fitness)]) {
        self.inner.clear();
        let mut acc = 0.0;
        for (g, f) in population {
            acc += (*f).into();
            self.inner.push((g.clone(), acc));
        }
        self.sum = acc;
    }

    fn choose(&self) -> G {
        let mut rng = self.rng.borrow_mut();
        let r: f64 = rng.gen::<f64>() * self.sum;
        let mut low = 0usize;
        let mut high = self.inner.len();
        while low < high {
            let mid = (low + high) / 2;
            if self.inner[mid].1 <= r {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        self.inner[low.min(self.inner.len() - 1)].0.clone()
    }
}

/// Tournament selection
pub struct TournamentSelector<G: GenoType> {
    size: usize,
    population: Vec<G>,
    rng: RefCell<StdRng>,
    _marker: std::marker::PhantomData<G>,
}

impl<G: GenoType> TournamentSelector<G> {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            population: Vec::new(),
            rng: RefCell::new(StdRng::from_entropy()),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_seed(size: usize, seed: u64) -> Self {
        Self {
            size,
            population: Vec::new(),
            rng: RefCell::new(StdRng::seed_from_u64(seed)),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<G: GenoType> Default for TournamentSelector<G> {
    fn default() -> Self { Self::new(2) }
}

impl<G> Roulette<G> for TournamentSelector<G>
where
    G: GenoType,
{
    fn reset(&mut self, population: &[(G, G::Fitness)]) {
        self.population = population.iter().map(|(g, _)| g.clone()).collect();
    }

    fn choose(&self) -> G {
        let mut rng = self.rng.borrow_mut();
        let mut best: Option<(G, G::Fitness)> = None;
        for _ in 0..self.size {
            let idx = rng.gen_range(0..self.population.len());
            let g = self.population[idx].clone();
            let f = g.fitness();
            match &best {
                Some((_, bf)) if *bf >= f => {},
                _ => best = Some((g, f)),
            }
        }
        best.unwrap().0
    }
}

/// Rank-based selector
pub struct RankSelector<G: GenoType>
where
    G::Fitness: Into<f64> + Copy,
{
    inner: Vec<(G, f64)>,
    sum: f64,
    rng: RefCell<StdRng>,
}

impl<G> RankSelector<G>
where
    G: GenoType,
    G::Fitness: Into<f64> + Copy,
{
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            sum: 0.0,
            rng: RefCell::new(StdRng::from_entropy()),
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            inner: Vec::new(),
            sum: 0.0,
            rng: RefCell::new(StdRng::seed_from_u64(seed)),
        }
    }
}

impl<G> Default for RankSelector<G>
where
    G: GenoType,
    G::Fitness: Into<f64> + Copy,
{
    fn default() -> Self { Self::new() }
}

impl<G> Roulette<G> for RankSelector<G>
where
    G: GenoType,
    G::Fitness: Into<f64> + Copy,
{
    fn reset(&mut self, population: &[(G, G::Fitness)]) {
        self.inner.clear();
        let mut items: Vec<(G, G::Fitness)> = population.iter().map(|(g, f)| (g.clone(), *f)).collect();
        items.sort_by(|(_, f1), (_, f2)| f1.partial_cmp(f2).unwrap_or(std::cmp::Ordering::Equal));
        let mut acc = 0.0;
        for (rank, (g, _)) in items.into_iter().enumerate() {
            let weight = (rank + 1) as f64; // 1..n
            acc += weight;
            self.inner.push((g, acc));
        }
        self.sum = acc;
    }

    fn choose(&self) -> G {
        let mut rng = self.rng.borrow_mut();
        let r: f64 = rng.gen::<f64>() * self.sum;
        let mut low = 0usize;
        let mut high = self.inner.len();
        while low < high {
            let mid = (low + high) / 2;
            if self.inner[mid].1 <= r {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        self.inner[low.min(self.inner.len() - 1)].0.clone()
    }
}

