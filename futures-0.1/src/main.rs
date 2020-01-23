#![deny(warnings)]
#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

mod client;
mod restaurant;

use env_logger::{self, Env};
use futures::Future;
use tokio;

fn run_restaurant(num_of_clients: usize) {
    let clients = client::get_clients(num_of_clients);
    let restaurant = restaurant::Restaurant;

    tokio::run(
        restaurant
            .serve(clients)
            .map_err(|err| panic!("Unexpected error {:#?}!", err)),
    );
}

fn main() {
    env_logger::from_env(Env::default().default_filter_or("debug")).init();
    run_restaurant(10);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn bench_run(b: &mut Bencher, num_of_clients: usize) {
        b.iter(|| run_restaurant(num_of_clients));
    }

    #[bench]
    fn bench_one(b: &mut Bencher) {
        bench_run(b, 1)
    }

    #[bench]
    fn bench_three(b: &mut Bencher) {
        bench_run(b, 3)
    }

    #[bench]
    fn bench_ten(b: &mut Bencher) {
        bench_run(b, 10)
    }
}
