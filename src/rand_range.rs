pub mod rand_range {
    use rand::{thread_rng, Rng};

    pub fn generate_random_range(upper: usize) -> Vec<i32> {
        let mut vec: Vec<i32> = (0..(upper as i32)).collect();
        let mut rng = thread_rng();
        rng.shuffle(&mut *vec);

        vec
    }
}

#[test]
fn test_check_vector_sizes() {
    let size_10 = rand_range::generate_random_range(10);
    assert_eq!(size_10.len(), 10);

    let size_million = rand_range::generate_random_range(1000000);
    assert_eq!(size_million.len(), 1000000);
}

#[test]
fn test_ensure_not_equal() {
    let vec_one = rand_range::generate_random_range(1000000);
    let vec_two = rand_range::generate_random_range(1000000);

    let mut idx: usize = 0;
    let mut equal = true;

    while idx < 1000000 {
        if vec_one[idx] != vec_two[idx] {
            equal = false;
        }
        idx += 1;
    }

    assert_eq!(equal, false);
}
