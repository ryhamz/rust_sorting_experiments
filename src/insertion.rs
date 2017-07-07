pub mod insertion {
    pub fn insertion_sort(v: &mut Vec<i32>) {
        let mut j: i32;
        for i in 1..v.len() {
            let x = v[i];
            j = (i - 1) as i32;

            while j >= 0 && v[j as usize] > x {
                v[(j+1) as usize] = v[j as usize];
                j -= 1;
            }
            v[(j+1) as usize] = x;
        }
    }
}

#[test]
fn test_sort_small_vector() {
    let mut v = vec![4, 8, 2, 6, 3, 1, 0, 9, 5, 7];

    insertion::insertion_sort(&mut v);

    let mut idx = 0;

    for i in 0..v.len() {
        assert_eq!(
            idx, v[i],
            "{} and {} are not equal",
            idx, v[i]
        );
        idx += 1;
    }
}

#[test]
fn test_randomly_generated_vector() {
}
