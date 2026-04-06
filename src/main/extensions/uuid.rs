use log::trace;
use rand::Rng;
use rand_xorshift::XorShiftRng;
use screeps::game;
use uuid::Uuid;
use crate::extensions::random::ScreepsRng;

pub trait UuidScreeps {
    /// A Screeps friendly UUID v4 generator that uses game time and CPU usage as a seed for a simple PRNG.
    fn new_screeps_v4() -> Self;
}

impl UuidScreeps for Uuid {
    fn new_screeps_v4() -> Self {
        let mut rng = XorShiftRng::from_game_time_shared(); // use share instance for more uniqueness

        let mut uuid_bytes = [0u8; 16];
        rng.fill_bytes(&mut uuid_bytes);
        
        Uuid::from_bytes(uuid_bytes) // err tick ends because no more cpu
    }
}