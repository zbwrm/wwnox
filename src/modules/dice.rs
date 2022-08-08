use rand::Rng;

pub fn d6() -> u8 {
    rand::thread_rng().gen_range(1..=6)
}
