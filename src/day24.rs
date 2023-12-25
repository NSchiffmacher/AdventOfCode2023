use std::fs::read_to_string;
use std::io::{self, Write};

use std::fmt::Display;
use itertools::Itertools;

use z3::ast::{Ast, Int, Real};

pub struct Solution {
    lines: Vec<String>,
    particles: Vec<Particle>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day24.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let particles = lines.iter().map(|l| Particle::from(l.as_str())).collect_vec();

        Self {
            lines,
            particles,
        }
    }

    fn part1(&mut self) -> usize {
        let mut res = 0;
        for i in 0..self.particles.len() {
            for j in i+1..self.particles.len() {
                if let Some((x, y)) = self.particles[i].intersect_2d(&self.particles[j]) {
                    if self.in_bounds((x, y)) {
                        res += 1;
                    }
                }
            }
        }
        res
    }

    fn in_bounds(&self, (x, y): (f64, f64)) -> bool {
        let min = 200000000000000.;
        let max = 400000000000000.;
        x >= min && y >= min && x <= max && y <= max
    }

    fn part2(&mut self) -> i64 {
        let ctx = z3::Context::new(&z3::Config::new());
        let s = z3::Solver::new(&ctx);
        let [fx,fy,fz,fdx,fdy,fdz] = ["fx","fy","fz","fdx","fdy","fdz"].map(|v| Real::new_const(&ctx, v));
      
        let zero = Int::from_i64(&ctx, 0).to_real();
        // Need only the first 3 particles
        for (i, part) in self.particles.iter().take(3).enumerate() {
          let [x,y,z,dx,dy,dz] = [part.x,part.y,part.z,part.vx,part.vy,part.vz].map(|v| Int::from_i64(&ctx, v as _).to_real());
          let t = Real::new_const(&ctx, format!("t{i}"));
          s.assert(&t.ge(&zero));
          s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
          s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
          s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
        }
        assert_eq!(s.check(), z3::SatResult::Sat);
        let res = s.get_model().unwrap().eval(&(&fx + &fy + &fz), true).unwrap();

        res.as_real().unwrap().0
    }

    pub fn solve(&mut self) {
        println!("========= DAY 24 ========");
        
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();

        let start = std::time::Instant::now();
        let part1 = self.part1();
        let part1_time = start.elapsed();
        println!("{:?} (took {:?})", part1, part1_time);

        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        let start = std::time::Instant::now();
        let part2 = self.part2();
        let part2_time = start.elapsed();
        println!("{:?} (took {:?})", part2, part2_time);
        println!();
    }
}

#[derive(Debug, Clone)]
struct Particle {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl From<&str> for Particle {
    fn from(value: &str) -> Self {
        let (position, speed) = value.split_once(" @ ").unwrap();
        let (x, y, z) = position.split(", ").map(|v| v.trim().parse::<f64>().unwrap()).collect_tuple().unwrap();
        let (vx, vy, vz) = speed.split(", ").map(|v| v.trim().parse::<f64>().unwrap()).collect_tuple().unwrap();
        Self { x, y, z, vx, vy, vz }
    }
}

impl Display for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {} ({}, {}, {})", self.x, self.y, self.z, self.vx, self.vy, self.vz)
    }
}

impl Particle {
    fn intersect_2d(&self, other: &Self) -> Option<(f64, f64)>{
        let den = self.vx * other.vy - self.vy * other.vx;
        if den == 0.0 {
            // println!("Parallel");
            return None;
        }

        let t = ((other.x - self.x) * other.vy + (self.y - other.y) * other.vx) / den;
        let u = ((other.x - self.x) * self.vy + (self.y - other.y) * self.vx) / den;

        if t < 0. || u < 0. {
            // println!("Intersect in the past");
            return None;
        }

        let x = self.x + t * self.vx;
        let y = self.y + t * self.vy;

        Some((x, y))
    }
}