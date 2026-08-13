use clap::Parser;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng as _};
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::PathBuf;

const EARTH_RAD: f64 = 6372.8;

#[derive(clap::Parser)]
pub struct Cli {
    count: usize,
    clusters: usize,
    seed: u64,
    output: PathBuf,
}
fn main() {
    let args = Cli::parse();
    let mut res = String::new();
    let mut total = 0.0;
    writeln!(res, "{{\"pairs\":[").unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(args.output)
        .unwrap();
    let mut rng = StdRng::seed_from_u64(args.seed);
    let mut clusters = Vec::with_capacity(args.clusters);
    for _ in 0..args.clusters {
        let x = rng.random_range(-157.0..175.0);
        let y = rng.random_range(-85.0..85.0);
        clusters.push((x, y));
    }
    for i in 0..args.count {
        if i != 0 {
            writeln!(res, ",").unwrap();
        }
        let cluster_0 = clusters[rng.random_range(0..args.clusters)];
        let cluster_1 = clusters[rng.random_range(0..args.clusters)];
        let x0 = cluster_0.0 + rng.random_range(-5.0..5.0);
        let y0 = cluster_0.1 + rng.random_range(-5.0..5.0);
        let x1 = cluster_1.0 + rng.random_range(-5.0..5.0);
        let y1 = cluster_1.1 + rng.random_range(-5.0..5.0);
        write!(res, "\t{{\"x0\":{x0},\"y0\":{y0},\"x1\":{x1},\"y1\":{y1}}}",).unwrap();

        total += haversine(x0, y0, x1, y1);
    }

    writeln!(res, "\n]}}").unwrap();

    writeln!(file, "{res}").unwrap();

    println!(
        "count: {}, total: {}, avg: {}",
        args.count,
        total,
        total / (args.count as f64)
    )
}

fn haversine(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    let lat1 = y0;
    let lat2 = y1;
    let lon1 = x0;
    let lon2 = x1;

    let lat_diff = (lat2 - lat1).to_radians();
    let lon_diff = (lon2 - lon1).to_radians();

    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();

    let a =
        (lat_diff / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (lon_diff / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    EARTH_RAD * c
}
