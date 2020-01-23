use std::fmt::Debug;

use futures::join;
use log::debug;

use super::food::{self, Food};

#[derive(Copy, Clone, Debug)]
pub enum Menu {
    DragonRoll,
    CutRoll,
}

impl Menu {
    pub async fn order(self) -> FullfiledOrder {
        debug!("Ordering {:?}", self);
        match self {
            Menu::DragonRoll => {
                let shrimp = food::raw::Shrimp.prepare();
                let shrimp_tempura = food::ingredient::ShrimpTempura(shrimp, food::raw::Tempura)
                    .prepare()
                    .cook();
                let rice = food::raw::Rice.prepare().cook();

                let (shrimp_tempura, rice) = join!(shrimp_tempura, rice);

                let order = food::sushi::DragonRoll(shrimp_tempura, rice, food::raw::Nori)
                    .prepare()
                    .cook()
                    .await;

                FullfiledOrder(Box::new(order))
            }
            Menu::CutRoll => {
                let fish = food::raw::Fish.prepare();
                let rice = food::raw::Rice.prepare().cook().await;

                let order = food::sushi::CutRoll(fish, rice, food::raw::Nori)
                    .prepare()
                    .cook()
                    .await;

                FullfiledOrder(Box::new(order))
            }
        }
    }
}

pub struct FullfiledOrder(Box<dyn Debug>);
impl FullfiledOrder {
    pub fn pay(self) {
        debug!("Payed for {:?}", &*self.0);
    }
}
