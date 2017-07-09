pub mod merge {
    pub fn merge_sort(v: &mut Vec<i32>) {
        let right = v.len();
        let mut v2: Vec<i32> = vec![0; right];
        merge_split(v, 0, right, &mut v2);
    }

    fn merge_split(v: &mut Vec<i32>, left: usize, right: usize, v2: &mut Vec<i32>) {
        if right - left <= 1 {
            return;
        } else {
            let mid = (left + right) / 2;
            merge_split(v, left, mid, v2);
            merge_split(v, mid, right, v2);
            merge(v, left, mid, right, v2);
            merge_copy(v, left, right, v2);
        }
    }

    fn merge_copy(v: &mut Vec<i32>, left: usize, right: usize, v2: &mut Vec<i32>) {
        for i in left..right {
            v[i] = v2[i];
        }
    }

    fn merge(v: &mut Vec<i32>, left: usize, mid: usize, right: usize, v2: &mut Vec<i32>) {
        let mut ptr1 = left;
        let mut ptr2 = mid;

        for i in left..right {
            if ptr1 < mid && (ptr2 >= right || v[ptr1] <= v[ptr2]) {
                v2[i] = v[ptr1];
                ptr1 += 1;
            } else {
                v2[i] = v[ptr2];
                ptr2 += 1;
            }
        }
    }
}

#[test]
fn test_sort_small_vector() {
    let mut v = vec![4, 8, 2, 6, 3, 1, 0, 9, 5, 7];

    merge::merge_sort(&mut v);

    let mut idx = 0;

    for i in 0..v.len() {
        assert_eq!(idx, v[i]);
        idx += 1;
    }
}

#[test]
fn test_randomly_generated_vector() {
    use rand_range::*;

    let mut rand_vec = rand_range::generate_random_range(1000);

    merge::merge_sort(&mut rand_vec);

    let mut idx: i32 = 0;

    while idx < 1000 {
        assert_eq!(idx, rand_vec[idx as usize]);
        idx += 1;
    }
}
