use evo::*;
use evo::pool::Pool;
use evo::pool::Ratios;
use evo::neat::NeatBreeder;
use evo::neat::NeatNetwork;

#[test]
fn test_xor() {
    let mut pool = Pool::new(150, NeatBreeder::new(2, 1));
    pool.ratios = Ratios {
        top: 0.1,
        mutate: 0.4,
        cross: 0.4,
        random: 0.1,
    };

    pool.run(10000, 10000, |mut n: NeatNetwork| {
        let mut d = 0.0f64;
        if let Some(mut x) = n.run(vec![0.0, 0.0], 1) {
            d += (1.0 - x.remove(0)).powi(2);
        }
        if let Some(mut x) = n.run(vec![1.0, 1.0], 1) {
            d += (1.0 - x.remove(0)).powi(2);
        }
        if let Some(mut x) = n.run(vec![1.0, 0.0], 1) {
            d += (0.0 - x.remove(0)).powi(2);
        }
        if let Some(mut x) = n.run(vec![0.0, 1.0], 1) {
            d += (0.0 - x.remove(0)).powi(2);
        }
        let fitness = 16f64 / (1f64 + d);
        fitness as f32
    });

}
