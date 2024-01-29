use {
    core::hash::Hasher,
    fastrand::Rng,
    spin::{Mutex, Once},
    uuid::Uuid,
};

static RNG: Once<Mutex<Rng>> = Once::INIT;

const OUT_DIR: &'static str = env!("OUT_DIR");

pub fn init() {
    let rng_seed = {
        let mut hasher = twox_hash::XxHash64::default();
        hasher.write(OUT_DIR.as_bytes());
        hasher.finish()
    };

    log::trace!("initializing rng with seed {:#x?}", rng_seed);

    RNG.call_once(|| Mutex::new(Rng::with_seed(rng_seed)));
}

pub fn new_uuid_v4() -> Uuid {
    let mut buf = [0u8; 16];
    RNG.get().unwrap().lock().fill(&mut buf);
    uuid::Builder::from_random_bytes(buf).into_uuid()
}
