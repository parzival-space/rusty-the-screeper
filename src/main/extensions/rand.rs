use js_sys::Date;
use log::trace;
use rand::{SeedableRng, TryRng};
use rand_xorshift::XorShiftRng;
use std::cell::RefCell;
use std::convert::Infallible;
use std::option::Option;
use wasm_bindgen::prelude::wasm_bindgen;

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
        let mut seed = <ScreepsXorShiftRng as SeedableRng>::Seed::default();

        // use current time as seed
        let current_date = Date::now();
        for chunk in seed.as_mut().chunks_exact_mut(8) {
            chunk.copy_from_slice(&current_date.to_le_bytes());
        }

        Self::from_seed(seed)
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

#[wasm_bindgen(js_name = "test")]
pub fn test_js() -> usize {
    // get current time
    Date::now() as usize
}