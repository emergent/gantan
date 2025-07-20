use gantan::selection::{FitnessProportionate, TournamentSelector, RankSelector};
use gantan::{GenoType, Roulette};

#[derive(Clone)]
struct FG(i32);

impl GenoType for FG {
    type Fitness = i32;
    type PhenoType = i32;

    fn fitness(&self) -> Self::Fitness { self.0 }
    fn decode(&self) -> Self::PhenoType { self.0 }
    fn mutate(&mut self) {}
    fn crossover(_g1:&mut Self,_g2:&mut Self){}
}

#[test]
fn fitness_proportionate_returns_member() {
    let genes = vec![FG(1), FG(2), FG(3)];
    let mut sel = FitnessProportionate::with_seed(42);
    sel.reset(&[
        (genes[0].clone(), genes[0].fitness()),
        (genes[1].clone(), genes[1].fitness()),
        (genes[2].clone(), genes[2].fitness()),
    ]);
    for _ in 0..10 {
        let g = sel.choose();
        assert!(genes.iter().any(|x| x.0 == g.0));
    }
}

#[test]
fn tournament_selector_returns_member() {
    let genes = vec![FG(1), FG(2), FG(3)];
    let mut sel = TournamentSelector::with_seed(2, 123);
    sel.reset(&[
        (genes[0].clone(), genes[0].fitness()),
        (genes[1].clone(), genes[1].fitness()),
        (genes[2].clone(), genes[2].fitness()),
    ]);
    for _ in 0..10 {
        let g = sel.choose();
        assert!(genes.iter().any(|x| x.0 == g.0));
    }
}

#[test]
fn rank_selector_returns_member() {
    let genes = vec![FG(1), FG(2), FG(3)];
    let mut sel = RankSelector::with_seed(99);
    sel.reset(&[
        (genes[0].clone(), genes[0].fitness()),
        (genes[1].clone(), genes[1].fitness()),
        (genes[2].clone(), genes[2].fitness()),
    ]);
    for _ in 0..10 {
        let g = sel.choose();
        assert!(genes.iter().any(|x| x.0 == g.0));
    }
}

#[test]
fn rank_selector_bias() {
    let genes = vec![FG(1), FG(10)];
    let mut sel = RankSelector::with_seed(7);
    sel.reset(&[
        (genes[0].clone(), genes[0].fitness()),
        (genes[1].clone(), genes[1].fitness()),
    ]);
    let mut count_best = 0;
    for _ in 0..100 {
        let g = sel.choose();
        if g.0 == 10 { count_best += 1; }
    }
    assert!(count_best > 50); // higher ranked should be chosen more often
}
