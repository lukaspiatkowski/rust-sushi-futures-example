mod food;
mod menu;

use futures::{stream::StreamExt, Stream};

use crate::client::Client;
pub use menu::Menu;

pub struct Restaurant;
impl Restaurant {
    pub async fn serve(self, clients: impl Stream<Item = Client>) {
        clients
            .for_each_concurrent(/* concurrency limit */ 3, async move |client| {
                let fullfilled = client.0.order().await;
                fullfilled.pay()
            })
            .await
    }
}
