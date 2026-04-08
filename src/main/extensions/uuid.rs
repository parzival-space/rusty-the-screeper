use crate::extensions::rand::ScreepsXorShiftRng;
use rand::Rng;
use uuid::Uuid;

pub trait UuidScreeps {
    /// A Screeps friendly UUID v4 generator that uses game time and CPU usage as a seed for a simple PRNG.
    fn new_screeps_v4() -> Self;
}

impl UuidScreeps for Uuid {
    fn new_screeps_v4() -> Self {
        let mut uuid_bytes = [0u8; 16];

        ScreepsXorShiftRng::with(|instance| instance.fill_bytes(&mut uuid_bytes));
        
        Uuid::from_bytes(uuid_bytes) // err tick ends because no more cpu
    }
}