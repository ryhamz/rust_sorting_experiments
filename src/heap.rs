pub mod heap {
    use std::iter::{ range_step };

    pub fn heap_sort(v: &mut Vec<i32>) {
        let size = v.len();
        let start = size / 2 - 1;

        for i in range_step(start, -1, -1) {
            heapify(v, size, i);
        }

        // One by one, extract element from heap
        for i in range_step(size - 1, -1, -1) {
            v.swap(0, i);
            heapify(v, i, 0);
        }
    }

    fn heapify(v: &mut Vec<i32>, size: usize, node: usize) {
        let mut largest = node;
        let left = 2 * node + 1;
        let right = 2 * node + 2;

        // left child is larger than root
        if left < size && v[left] > v[largest] {
            largest = left;
        }

        // right child is larger than largest so far
        if right < node && v[right] > v[largest] {
            largest = right;
        }

        // largest is not root
        if largest != node {
            v.swap(node, largest);

            // recursively heapify the affected subtree
            heapify(v, size, largest);
        }
    }
}

#[test]
fn test_sort_small_vector() {
    let mut v = vec![4, 8, 2, 6, 3, 1, 0, 9, 5, 7];

    heap::heap_sort(&mut v);

    let mut idx = 0;

    for i in 0..v.len() {
        assert_eq!(idx, v[i]);
        idx += 1;
    }
}


