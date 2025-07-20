use gantan::{GenoType, Inspector, Population, Roulette, SimulatorBuilder};
use std::cell::Cell;

#[derive(Clone, Debug, PartialEq)]
struct TestGene(i32);

impl GenoType for TestGene {
    type Fitness = i32;
    type PhenoType = i32;

    fn fitness(&self) -> Self::Fitness {
        self.0
    }
    fn decode(&self) -> Self::PhenoType {
        self.0
    }
    fn mutate(&mut self) {
        self.0 += 1;
    }
    fn crossover(g1: &mut Self, g2: &mut Self) {
        std::mem::swap(&mut g1.0, &mut g2.0);
    }
}

#[derive(Default)]
struct CycleSelector {
    inner: Vec<TestGene>,
    idx: Cell<usize>,
}

impl Roulette<TestGene> for CycleSelector {
    fn reset(&mut self, population: &[(TestGene, <TestGene as GenoType>::Fitness)]) {
        self.inner = population.iter().map(|(g, _)| g.clone()).collect();
        self.idx.set(0);
    }
    fn choose(&self) -> TestGene {
        let i = self.idx.get();
        self.idx.set((i + 1) % self.inner.len());
        self.inner[i].clone()
    }
}

struct LenInspector {
    first_len: usize,
    checked: Cell<bool>,
}

impl Inspector<TestGene> for LenInspector {
    fn inspect(&mut self, _generation: usize, p: &Population<TestGene>) -> bool {
        if !self.checked.replace(true) {
            assert_eq!(p.len(), self.first_len);
        }
        false
    }
}

struct FixedRoulette {
    draws: Vec<f64>,
    index: Cell<usize>,
    inner: Vec<(TestGene, f64)>,
    sum: f64,
}

impl FixedRoulette {
    fn new(draws: Vec<f64>) -> Self {
        Self {
            draws,
            index: Cell::new(0),
            inner: Vec::new(),
            sum: 0.0,
        }
    }
}

impl Roulette<TestGene> for FixedRoulette {
    fn reset(&mut self, population: &[(TestGene, <TestGene as GenoType>::Fitness)]) {
        self.inner.clear();
        let mut last = 0.0;
        for (g, f) in population {
            last += *f as f64;
            self.inner.push((g.clone(), last));
        }
        self.sum = last;
        self.index.set(0);
    }

    fn choose(&self) -> TestGene {
        let i = self.index.get();
        self.index.set(i + 1);
        let r = self.draws[i % self.draws.len()];
        let fit_val = r * self.sum;
        let mut low = 0usize;
        let mut high = self.inner.len();
        while low != high {
            let mid = (low + high) / 2;
            if self.inner[mid].1 <= fit_val {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        self.inner[low].0.clone()
    }
}

#[test]
fn population_get_best() {
    let p = Population::from(vec![TestGene(1), TestGene(3), TestGene(2)]);
    assert_eq!(p.get_best().cloned(), Some(TestGene(3)));
}

#[test]
fn simulator_step_generation_keeps_size() {
    let genes = vec![TestGene(1), TestGene(2), TestGene(3), TestGene(4)];
    let population = Population::from(genes);
    let selector = CycleSelector::default();
    let mut builder = SimulatorBuilder::new();
    builder
        .with_population(population)
        .with_inspector(LenInspector {
            first_len: 4,
            checked: Cell::new(false),
        })
        .with_crossover_rate(0.0)
        .with_mutation_rate(0.0)
        .with_selector(selector);
    let mut sim = builder.build();
    sim.start();
}

#[test]
fn weighted_selector_deterministic() {
    let g1 = TestGene(1);
    let g2 = TestGene(2);
    let g3 = TestGene(3);
    let draws = vec![0.0, 0.3, 0.8];
    let mut sel = FixedRoulette::new(draws);
    sel.reset(&[
        (g1.clone(), g1.fitness()),
        (g2.clone(), g2.fitness()),
        (g3.clone(), g3.fitness()),
    ]);
    let g1 = sel.choose();
    let g2 = sel.choose();
    let g3 = sel.choose();
    assert_eq!(g1, TestGene(1));
    assert_eq!(g2, TestGene(2));
    assert_eq!(g3, TestGene(3));
}
