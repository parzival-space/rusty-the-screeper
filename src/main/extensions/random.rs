use std::option::Option;
use std::cell::RefCell;
use std::sync::Arc;
use log::{error, trace};
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use screeps::game;

thread_local! {
    static SCREEPS_RNG_XOR_SHIFT_RNG: RefCell<Option<Arc<XorShiftRng>>> = RefCell::new(None);
}

pub trait ScreepsRng {
    fn from_game_time() -> Self;

    /// Same as <see>from_game_time()</see>, but also keeps the random generator alive across
    /// game ticks.
    fn from_game_time_shared() -> Self;
}

impl ScreepsRng for XorShiftRng {
    fn from_game_time() -> Self {
        let game_time = game::time() as u64;
        let cpu = (game::cpu::get_used() * 1000.0) as u64;
        let seed = (game_time ^ (cpu << 32));
        Self::seed_from_u64(seed)
    }

    fn from_game_time_shared() -> Self {
        SCREEPS_RNG_XOR_SHIFT_RNG.with(|r| {
            let mut borrow = r.borrow_mut();
            let arc_rng = borrow.get_or_insert_with(|| {
                trace!("Creating new shared XorShiftRng instance for ScreepsRng::from_game_time_shared");
                Arc::new(Self::from_game_time())
            });

            // Mutate the original RNG, don't clone state
            // this is risky; better would be to return &mut XorShiftRng or wrap in Mutex,
            // but this is not possible here right now
            (**arc_rng).clone()
        })
    }
}