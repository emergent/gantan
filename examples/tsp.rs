use gantan::{GenoType, Inspector, PhenoType, Population, Roulette, Simulator};
use ordered_float::OrderedFloat;
use rand::prelude::*;

struct Pheno<'a> {
    cities: &'a [(i32, i32)],
    indices: Vec<usize>,
}

impl<'a> PhenoType for Pheno<'a> {
    type GenoType = Gene<'a>;

    fn encode(&self) -> Self::GenoType {
        let mut base: Vec<usize> = (0..self.indices.len()).into_iter().collect();
        let mut gene = vec![0; self.indices.len()];

        for (i, gene_idx) in gene.iter_mut().enumerate().take(self.indices.len()) {
            let mut idx = 0;
            for b in &base {
                if *b == self.indices[i] {
                    break;
                }
                idx += 1;
            }
            *gene_idx = idx;
            base.remove(idx);
        }

        Gene {
            cities: self.cities,
            gene,
        }
    }
}

impl<'a> Pheno<'a> {
    fn new(cities: &'a [(i32, i32)], indice: Vec<usize>) -> Self {
        Self {
            cities,
            indices: indice,
        }
    }

    #[allow(unused)]
    fn measure_distance(&self) -> f64 {
        let mut total = 0.0;
        let len = self.cities.len();

        for i in 0..len {
            let from = self.cities[(self.indices[i]) as usize];
            let to = self.cities[(self.indices[(i + 1) % len]) as usize];
            total += (((from.0 - to.0).pow(2) + (from.1 - to.1).pow(2)) as f64).sqrt();
        }

        total
    }
}

#[derive(Clone, Debug)]
struct Gene<'a> {
    cities: &'a [(i32, i32)],
    gene: Vec<usize>,
}

impl<'a> GenoType for Gene<'a> {
    type Fitness = OrderedFloat<f64>;
    type PhenoType = Pheno<'a>;

    fn fitness(&self) -> Self::Fitness {
        OrderedFloat::from(100000.0 / self.measure_distance())
    }

    fn decode(&self) -> Self::PhenoType {
        let mut base: Vec<usize> = (0..self.gene.len()).into_iter().collect();

        let mut indice = vec![];
        for i in 0..self.gene.len() {
            let p = base.remove((self.gene[i]) as usize);
            indice.push(p);
        }

        Pheno {
            cities: self.cities,
            indices: indice,
        }
    }

    fn mutate(&mut self) {
        let len = self.gene.len();

        let (pos, value) = loop {
            let p = rand::random::<usize>() % len;
            let v = rand::random::<usize>() % (len - p);
            if p != len - 1 && self.gene[p] != v {
                break (p, v);
            }
        };
        self.gene[pos] = value;
    }

    fn crossover(g1: &mut Self, g2: &mut Self) {
        let mut rng = rand::thread_rng();
        let mut pos = (0..g1.gene.len())
            .into_iter()
            .choose_multiple(&mut rng, 2)
            .into_iter()
            .collect::<Vec<usize>>();
        pos.sort_unstable();

        let tmp = g1.gene.clone();
        for (i, tmp_item) in tmp.iter().enumerate().take(pos[1]).skip(pos[0]) {
            g1.gene[i] = g2.gene[i];
            g2.gene[i] = *tmp_item;
        }
    }
}

impl<'a> Gene<'a> {
    fn measure_distance(&self) -> f64 {
        let mut total = 0.0;
        let mut cities = self.cities.to_owned();
        let len = self.gene.len();

        let mut p0 = (0, 0);
        let mut pb = (0, 0);
        for i in 0..len {
            let p = cities.remove(self.gene[i]);

            if i == 0 {
                p0 = p;
            } else {
                let from = pb;
                let to = p;
                total += (((from.0 - to.0).pow(2) + (from.1 - to.1).pow(2)) as f64).sqrt();
            }

            if i == len - 1 {
                let from = p;
                let to = p0;
                total += (((from.0 - to.0).pow(2) + (from.1 - to.1).pow(2)) as f64).sqrt();
            }
            pb = p;
        }

        total
    }
}

#[derive(Default)]
struct CityRoulette<'a> {
    inner: Vec<(Gene<'a>, f64)>,
    sum: f64,
}

impl<'a> Roulette<Gene<'a>> for CityRoulette<'a> {
    fn reset(&mut self, population: &[(Gene<'a>, <Gene<'a> as GenoType>::Fitness)]) {
        self.inner.clear();
        let mut last = 0.0;
        for (g, f) in population {
            let val = f.into_inner() + last;
            self.inner.push((g.to_owned(), val));
            last = val;
        }
        self.sum = last;
    }

    fn choose(&self) -> Gene<'a> {
        let mut rng = thread_rng();
        let r: f64 = rng.gen();
        let fit_val = r * self.sum;

        let len = self.inner.len();
        let mut window = len / 4;
        let mut idx = len / 2;
        loop {
            let (_, val) = self.inner[idx];
            if fit_val > val {
                if window == 0 {
                    idx += 1
                } else {
                    idx = std::cmp::min(idx + window + 1, len - 1);
                }
            } else if idx == 0 || fit_val > self.inner[idx - 1].1 {
                return self.inner[idx].0.clone();
            } else {
                if window == 0 {
                    idx = idx - 1;
                } else {
                    idx = idx - window;
                }
            }
            window /= 2;
        }
    }
}

struct Ins;

impl Inspector<Gene<'_>> for Ins {
    fn inspect(&mut self, generation: usize, _population: &Population<Gene>) -> bool {
        if generation % 100 == 0 {
            if let Some(g) = _population.get_best() {
                println!(
                    "len: {:.3}, fitness: {:.3}, {:?}",
                    g.measure_distance(),
                    g.fitness(),
                    g.decode().indices
                );
            }
        }

        generation != 5000
    }
}

fn main() {
    let cities = vec![
        (207, 206),
        (3, 220),
        (218, 224),
        (79, 112),
        (75, 101),
        (24, 240),
        (232, 254),
        (89, 65),
        (146, 218),
        (86, 63),
        (255, 129),
        (30, 16),
        (267, 270),
        (124, 223),
        (201, 255),
        (212, 273),
        (209, 180),
        (37, 5),
        (3, 256),
        (193, 225),
        (193, 113),
        (126, 273),
        (218, 44),
        (101, 61),
        (20, 104),
    ];

    let size = 5000;
    let mut rng = rand::thread_rng();
    let mut p = vec![];
    let v = (0..cities.len()).into_iter().collect::<Vec<usize>>();
    for _i in 0..size {
        let mut v1 = v.clone();
        v1.shuffle(&mut rng);
        p.push(Pheno::new(&cities, v1).encode());
    }

    let inspector = Ins;
    let selector = CityRoulette::default();

    let mut simulator = Simulator::new(Population::from(p), inspector, 0.8, 0.2, selector);
    simulator.start();
}
