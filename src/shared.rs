pub mod shared {
    use std::collections::HashSet;

    use parking_lot::Mutex;
    use rand::{rngs::StdRng, RngCore};

    pub type Coord = (u32, u32);

    pub fn add(u: u32, i: i32, size: u32) -> u32 {
        if u == 0 && i.is_negative() {
            size - i.wrapping_abs() as u32 % size
        } else {
            let result = if i.is_negative() {
                u.checked_sub(i.checked_abs().unwrap() as u32).unwrap()
            } else {
                u.checked_add(i as u32).unwrap()
            };

            if result >= size {
                result % size
            } else {
                result
            }
        }
    }

    pub fn generate_unique_ids(_rng: &Mutex<StdRng>, count: usize) -> HashSet<u32> {
        let mut unique_ids = HashSet::new();
        let mut rng = _rng.lock();

        while unique_ids.len() < count {
            let id = rng.next_u32();
            unique_ids.insert(id);
        }

        unique_ids
    }

    pub fn generate_unique_id(_rng: &Mutex<StdRng>) -> u32 {
        let mut rng = _rng.lock();

        rng.next_u32()
    }
}
