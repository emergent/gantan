use gantan::{GenoType, Population};
use std::time::Instant;

#[derive(Clone)]
struct HeavyGene(u64);

impl GenoType for HeavyGene {
    type Fitness = u64;
    type PhenoType = ();

    fn fitness(&self) -> Self::Fitness {
        let mut acc = 0u64;
        for i in 0..1000u64 {
            acc = acc.wrapping_add(self.0.wrapping_mul(i));
        }
        acc
    }

    fn decode(&self) -> Self::PhenoType {
        ()
    }

    fn mutate(&mut self) {}

    fn crossover(_g1: &mut Self, _g2: &mut Self) {}
}

fn main() {
    let genes: Vec<HeavyGene> = (0..10_000).map(HeavyGene).collect();
    let start = Instant::now();
    let _p = Population::from(genes);
    let elapsed = start.elapsed();
    println!("population built in {:?}", elapsed);
}
