pub mod merge {
    pub fn merge_sort(v: &mut Vec<i32>) {
        let right = (v.len() - 1) as i32;
        msort(v, 0, right);
    }

    fn msort(v: &mut Vec<i32>, left: i32, right: i32) {
        if left < right {
            //let mid = left + (right - left) / 2;
            let mid = (right + left) / 2;

            // sort first and second halves
            msort(v, left, mid);
            msort(v, mid + 1, right);

            merge(v, left, mid, right);
        }
    }

    fn merge(v: &mut Vec<i32>, left: i32, mid: i32, right: i32) {
        let left_u = left as usize;
        let mid_u = mid as usize;
        let right_u = right as usize;

        let n1 = mid - left + 1;
        let n2 = right - mid;
        let n1_u = n1 as usize;
        let n2_u = n2 as usize;

        let mut left_arr: Vec<i32> = Vec::new();
        let mut right_arr: Vec<i32> = Vec::new();

        // Copy data to temp arrays
        for i in 0..(n1 as usize) {
            left_arr.push(v[left_u + i]);
        }

        for j in 0..(n2 as usize) {
            right_arr.push(v[mid_u + 1 + j]);
        }

        // Merge temp arrays back into v[1..right]
        let mut i: usize = 0; // initial index of first subarray
        let mut j: usize = 0; // initial index of second subarray
        let mut k: usize = 1; // initial index of merged subarray

        while i < n1_u && j < n2_u {
            if left_arr[i] <= right_arr[i] {
                v[k] = left_arr[i];
                i += 1;
            } else {
                v[k] = right_arr[j];
                j += 1;
            }
            k += 1;
        }

        // Copy any remaining elements of left_arr
        while i < n1_u {
            v[k] = left_arr[i];
            i += 1;
            k += 1;
        }

        // Copy any remaining elements of right_arr
        while j < n2_u {
            v[k] = right_arr[j];
            j += 1;
            k += 1;
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
