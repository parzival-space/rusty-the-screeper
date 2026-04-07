use std::option::Option;
use std::cell::RefCell;
use std::convert::Infallible;
use log::{trace};
use rand::{Rng, SeedableRng, TryRng};
use rand_xorshift::XorShiftRng;
use screeps::game;
use crate::creeps::manager::CreepManager;

thread_local! {
    static STORE: RefCell<Option<ScreepsXorShiftRng>> = RefCell::new(None);
}

#[derive(Debug)]
pub struct ScreepsXorShiftRng {
    rng: XorShiftRng,
}

impl SeedableRng for ScreepsXorShiftRng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            rng: XorShiftRng::from_seed(seed),
        }
    }
}

impl TryRng for ScreepsXorShiftRng {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        self.rng.try_next_u32()
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        self.rng.try_next_u64()
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        self.rng.try_fill_bytes(dst)
    }
}

impl ScreepsXorShiftRng {
    fn new() -> Self {
        let game_time = game::time() as u64;
        let cpu = (game::cpu::get_used() * 1000.0) as u64;
        let seed = (game_time ^ (cpu << 32));
        Self::seed_from_u64(seed)
    }

    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&mut ScreepsXorShiftRng) -> R,
    {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            
            let instance = store.get_or_insert_with(|| {
                trace!("Creating new ScreepsXorShiftRng instance for ScreepsXorShiftRng::with");
                Self::new()
            });
            
            f(instance)
        })
    }
}