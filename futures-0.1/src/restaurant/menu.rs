use std::fmt::Debug;

use anyhow::Error;
use futures::future::{Either, Future};
use log::debug;

use super::food::{self, Food};

#[derive(Copy, Clone, Debug)]
pub enum Menu {
    DragonRoll,
    CutRoll,
}

impl Menu {
    pub fn order(self) -> impl Future<Item = FullfiledOrder, Error = Error> {
        debug!("Ordering {:?}", self);
        match self {
            Menu::DragonRoll => {
                let shrimp = food::raw::Shrimp.prepare();
                let shrimp_tempura = food::ingredient::ShrimpTempura(shrimp, food::raw::Tempura)
                    .prepare()
                    .cook();
                let rice = food::raw::Rice.prepare().cook();

                Either::A(
                    shrimp_tempura
                        .join(rice)
                        .and_then(|(shrimp_tempura, rice)| {
                            food::sushi::DragonRoll(shrimp_tempura, rice, food::raw::Nori)
                                .prepare()
                                .cook()
                        })
                        .map(|order| FullfiledOrder(Box::new(order))),
                )
            }
            Menu::CutRoll => {
                let fish = food::raw::Fish.prepare();
                let rice = food::raw::Rice.prepare().cook();

                Either::B(
                    rice.and_then(|rice| {
                        food::sushi::CutRoll(fish, rice, food::raw::Nori)
                            .prepare()
                            .cook()
                    })
                    .map(|order| FullfiledOrder(Box::new(order))),
                )
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
