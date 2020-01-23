use anyhow::Error;
use futures::{stream, Stream};
use quickcheck::{Arbitrary, Gen, StdThreadGen};
use rand::seq::SliceRandom;

use crate::restaurant::Menu;

#[derive(Clone)]
pub struct Client(pub Menu);

impl Arbitrary for Client {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let choices = [Menu::DragonRoll, Menu::CutRoll];
        Self(*choices.choose(g).expect("Choices shouldn't be empty"))
    }
}

pub fn get_clients(num_of_clients: usize) -> impl Stream<Item = Client, Error = Error> {
    stream::unfold(num_of_clients, |clients_left| {
        if clients_left > 0 {
            let mut gen = StdThreadGen::new(10);
            Some(Ok((Client::arbitrary(&mut gen), clients_left - 1)))
        } else {
            None
        }
    })
}
