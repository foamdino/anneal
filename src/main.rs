extern crate rand;
use crate::rand::Rng;

use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Clone, Copy)]
struct City {
    x: i16,
    y: i16,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

fn euclidean_distance(from: &City, to: &City) -> f64 {
    let dx: i32 = (from.x - to.x).abs() as i32;
    let dy: i32 = (from.y - to.y).abs() as i32;
    (((dx*dx) + (dy*dy)) as f64).sqrt()
}

fn tour_distance(cities: &Vec<City>) -> f64 {
    let mut tour_distance = 0.0;
    for i in 0..cities.len() {
        let src = cities.get(i).unwrap();

        let dest = if i + 1 < cities.len() {
            cities.get(i + 1).unwrap()
        } else {
            cities.get(0).unwrap()
        };

        tour_distance += euclidean_distance(src, dest);
    }
    tour_distance
}

fn acceptance_probability(e1: f64, e2: f64, temp: f64) -> f64 {
    if e2 < e1 {
        1.0
    } else {
        ((e1 - e2) / temp).exp()
    }
}

fn main() {
    let mut cities: Vec<City> = vec![];
    cities.push(City{x: 60, y: 200});
    cities.push(City{x: 180, y: 200});
    cities.push(City{x: 80, y: 180});
    cities.push(City{x: 140, y: 180});
    cities.push(City{x: 20, y: 160});
    cities.push(City{x: 100, y: 160});
    cities.push(City{x: 200, y: 160});
    cities.push(City{x: 140, y: 140});
    cities.push(City{x: 40, y: 120});
    cities.push(City{x: 100, y: 120});
    cities.push(City{x: 180, y: 100});
    cities.push(City{x: 60, y: 80});
    cities.push(City{x: 120, y: 80});
    cities.push(City{x: 180, y: 60});
    cities.push(City{x: 20, y: 40});
    cities.push(City{x: 100, y: 40});
    cities.push(City{x: 200, y: 40});
    cities.push(City{x: 20, y: 20});
    cities.push(City{x: 60, y: 20});
    cities.push(City{x: 160, y: 20});

    let mut temp = 10000.0;
    let cooling_rate = 0.003;
    let mut rng = thread_rng();
    let len = cities.len();
    let mut baseline= cities.clone();
    baseline.shuffle(&mut thread_rng());

    let mut best = baseline.clone();

    println!("Initial baseline solution: {}", tour_distance(&baseline));

    while temp > 1.0 {
        let mut trial = best.clone();
        let pos1 = rng.gen_range(0, len);
        let pos2 = rng.gen_range(0, len);

        // swap cities at pos1 & pos2
        trial.swap(pos1, pos2);

        // get energy of each solution
        let e1 = tour_distance(&best);
        let e2 = tour_distance(&trial);

        if acceptance_probability(e1, e2, temp) > rng.gen::<f64>() {
            best = trial;
        }

        // cool the system
        temp *= 1.0-cooling_rate;
    }

    println!("Final solution: {}", tour_distance(&best));
    println!("{:?}", best);
}
