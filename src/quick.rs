pub mod quick {
    pub fn quick_sort(v: &mut Vec<i32>) {
        let len = (v.len() - 1) as i32;
        qsort(v, 0, len);
    }

    fn qsort(v: &mut Vec<i32>, lo: i32, hi: i32) {
        if lo < hi {
            let pt = partition(v, lo, hi);
            qsort(v, lo, pt - 1);
            qsort(v, pt + 1, hi);
        }
    }

    fn partition(v: &mut Vec<i32>, lo: i32, hi: i32) -> i32 {
        let lo_u = lo as usize;
        let hi_u = hi as usize;
        // Pivot: element to be placed at right position
        let pivot = v[hi as usize];

        // Index of smaller element
        let mut i = lo - 1;

        for j in lo_u..hi_u {
            // if current element is smaller than
            // or equal to pivot
            if v[j] <= pivot {
                i += 1;
                v.swap(i as usize, j);
            }
        }
        v.swap((i + 1) as usize, hi_u);
        return i + 1;
    }
}

#[test]
fn test_sort_small_vector() {
    let mut v = vec![4, 8, 2, 6, 3, 1, 0, 9, 5, 7];

    quick::quick_sort(&mut v);

    let mut idx = 0;

    for i in 0..v.len() {
        assert_eq!(idx, v[i]);
        idx += 1;
    }
}
