pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        arr.swap(i, min);
    }
}

pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let mut aux = arr.to_vec();
    let n = arr.len();
    merge_sort_helper(arr, &mut aux, 0, n - 1);
}

fn merge_sort_helper<T: Ord + Copy>(arr: &mut [T], aux: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    merge_sort_helper(arr, aux, lo, mid);
    merge_sort_helper(arr, aux, mid + 1, hi);
    merge(arr, aux, lo, mid, hi);
}

fn merge<T: Ord + Copy>(arr: &mut [T], aux: &mut [T], lo: usize, mid: usize, hi: usize) {
    for k in lo..=hi {
        aux[k] = arr[k];
    }
    let mut i = lo;
    let mut j = mid + 1;
    for k in lo..=hi {
        if i > mid {
            arr[k] = aux[j];
            j += 1;
        } else if j > hi {
            arr[k] = aux[i];
            i += 1;
        } else if aux[j] < aux[i] {
            arr[k] = aux[j];
            j += 1;
        } else {
            arr[k] = aux[i];
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![3, 2, 1, 4, 7, 8, 5, 9, 6];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![3, 2, 1, 4, 7, 8, 5, 9, 6];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![3, 2, 1, 4, 7, 8, 5, 9, 6];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
