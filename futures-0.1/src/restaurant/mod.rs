mod food;
mod menu;

use anyhow::Error;
use futures::{Future, Stream};

use crate::client::Client;
pub use menu::Menu;

pub struct Restaurant;
impl Restaurant {
    pub fn serve(
        self,
        clients: impl Stream<Item = Client, Error = Error>,
    ) -> impl Future<Item = (), Error = Error> {
        clients
            .map(|client| client.0.order().map(|fullfilled| fullfilled.pay()))
            .buffer_unordered(3)
            .for_each(|()| Ok(()))
    }
}
