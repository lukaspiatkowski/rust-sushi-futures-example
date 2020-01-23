use std::fmt::Debug;
use std::thread;
use std::time::Duration;

use log::debug;
use tokio::time::delay_for;

pub trait Food: Debug {
    const PREPARE_TIME: Duration;
    const COOK_TIME: Duration;

    fn prepare(self) -> Prepared<Self>
    where
        Self: Sized,
    {
        debug!("Preparing {:?}", self);
        // preparing by holding the thread
        thread::sleep(Self::PREPARE_TIME);
        debug!("Done preparing {:?}", self);
        Prepared(self)
    }
}

#[derive(Debug)]
pub struct Prepared<T>(T);
impl<T: Food + 'static> Prepared<T> {
    pub async fn cook(self) -> Cooked<T> {
        debug!("Cooking {:?}", self.0);
        delay_for(T::COOK_TIME).await;
        debug!("Done cooking {:?}", self.0);
        Cooked(self.0)
    }
}

#[derive(Debug)]
pub struct Cooked<T>(T);

pub mod raw {
    use super::*;

    #[derive(Debug)]
    pub struct Fish;
    impl Food for Fish {
        const PREPARE_TIME: Duration = Duration::from_millis(2);
        const COOK_TIME: Duration = Duration::from_millis(3);
    }

    #[derive(Debug)]
    pub struct Shrimp;
    impl Food for Shrimp {
        const PREPARE_TIME: Duration = Duration::from_millis(1);
        const COOK_TIME: Duration = Duration::from_millis(2);
    }

    #[derive(Debug)]
    pub struct Rice;
    impl Food for Rice {
        const PREPARE_TIME: Duration = Duration::from_millis(0);
        const COOK_TIME: Duration = Duration::from_millis(5);
    }

    #[derive(Debug)]
    pub struct Tempura;
    #[derive(Debug)]
    pub struct Nori;
}

pub mod ingredient {
    use super::*;

    #[derive(Debug)]
    pub struct ShrimpTempura(pub Prepared<raw::Shrimp>, pub raw::Tempura);
    impl Food for ShrimpTempura {
        const PREPARE_TIME: Duration = Duration::from_millis(0);
        const COOK_TIME: Duration = Duration::from_millis(5);
    }
}

pub mod sushi {
    use super::*;

    #[derive(Debug)]
    pub struct DragonRoll(
        pub Cooked<ingredient::ShrimpTempura>,
        pub Cooked<raw::Rice>,
        pub raw::Nori,
    );
    impl Food for DragonRoll {
        const PREPARE_TIME: Duration = Duration::from_millis(2);
        const COOK_TIME: Duration = Duration::from_millis(0);
    }

    #[derive(Debug)]
    pub struct CutRoll(
        pub Prepared<raw::Fish>,
        pub Cooked<raw::Rice>,
        pub raw::Nori,
    );
    impl Food for CutRoll {
        const PREPARE_TIME: Duration = Duration::from_millis(4);
        const COOK_TIME: Duration = Duration::from_millis(0);
    }
}
