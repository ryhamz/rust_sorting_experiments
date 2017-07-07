extern crate rand;

use rand::Rng;

pub mod rand_range {
    pub fn generate_random_range(upper: usize) -> Vec<i32> {
        let mut vec: Vec<i32> = (0..(upper as i32)).collect();
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut *vec);

        vec
    }
}

#[test]
fn test_check_vector_sizes() {
    let size_10 = rand_range::generate_random_range(10);
    assert_eq!(size_10.len(), 10);
}
