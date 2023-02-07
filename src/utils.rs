use std::cmp::PartialOrd;

pub(crate) fn kth_smallest<T: PartialOrd>(
    data: &mut [Vec<T>],
    k: usize,
    cut_axis: usize,
) -> Result<usize, String> {
    if k == 0 {
        return Err("k should be greater than 0, but you give 0.".to_string());
    }

    let size = data.len();
    if size < k {
        return Err(format!("the size of data [{size}] is less than k [{k}]"));
    }

    let mut res_idx = _quick_select(data, 0, size - 1, k, cut_axis);

    // make sure elements in @data with index < res_idx is less than @data[res_idx]
    //       and elements in @data with index > res_idx is greater or equal than @data[res_idx]
    while res_idx > 0 && data[res_idx - 1][cut_axis] == data[res_idx][cut_axis] {
        res_idx -= 1;
    }

    Ok(res_idx)
}

fn _quick_select<T: PartialOrd>(
    data: &mut [Vec<T>],
    l: usize,
    r: usize,
    k: usize,
    cut_axis: usize,
) -> usize {
    if l >= r {
        return l;
    }

    let mut i = l as isize - 1;
    let mut j = r as isize + 1;
    // let x = data[indices[(l + r) >> 1]];
    // let x = data[l];
    let pivot = l;

    while i < j {
        loop {
            i += 1;
            if data[i as usize][cut_axis] >= data[pivot][cut_axis] {
                break;
            }
        }
        loop {
            j -= 1;
            if data[j as usize][cut_axis] <= data[pivot][cut_axis] {
                break;
            }
        }
        if i < j {
            data.swap(i as usize, j as usize);
        }
    }

    if k <= j as usize - l + 1 {
        return _quick_select(data, l, j as usize, k, cut_axis);
    }
    _quick_select(data, j as usize + 1, r, k - (j as usize - l + 1), cut_axis)
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    // check if element with index < @idx is less than @data[idx]
    //      and element with index > @idx is greater or equal than @data[idx]
    fn check<T: PartialOrd>(data: &[Vec<T>], idx: usize, cut_axis: usize) {
        for i in 0..idx {
            assert!(data[i][cut_axis] < data[idx][cut_axis]);
        }
        for i in idx..data.len() {
            assert!(data[i][cut_axis] >= data[idx][cut_axis]);
        }
    }

    #[test]
    fn basic() {
        let mut a = [
            vec![10],
            vec![8],
            vec![4],
            vec![3],
            vec![1],
            vec![9],
            vec![2],
            vec![7],
            vec![5],
            vec![6],
        ];

        // baic
        let cut_axis = 0;
        let idx = kth_smallest(&mut a, 3, cut_axis).unwrap();
        assert_eq!(a[idx][cut_axis], 3);
        check(&a, idx, cut_axis);
    }

    #[test]
    fn duplicate_elemnts() {
        let mut a = [vec![2], vec![2], vec![2]];

        let cut_axis = 0;
        let idx = kth_smallest(&mut a, 3, cut_axis).unwrap();
        assert_eq!(idx, 0);
        check(&a, idx, cut_axis);
    }

    #[test]
    fn single_element() {
        let mut a = [vec![2]];

        let cut_axis = 0;
        let idx = kth_smallest(&mut a, 1, cut_axis).unwrap();
        assert_eq!(a[idx][cut_axis], 2);
        check(&a, idx, cut_axis);
    }

    #[test]
    fn presorted() {
        let mut a = [vec![1], vec![2], vec![3], vec![8]];

        let cut_axis = 0;
        let idx = kth_smallest(&mut a, 4, cut_axis).unwrap();
        assert_eq!(a[idx][cut_axis], 8);
        check(&a, idx, cut_axis);
    }

    #[test]
    fn error() {
        // list size is less than k
        let mut a: [Vec<i32>; 0] = [];
        assert!(kth_smallest(&mut a, 3, 1).is_err());

        let mut a = [vec![1]];
        assert!(kth_smallest(&mut a, 10, 1).is_err());

        // k = 0
        assert!(kth_smallest(&mut a, 10, 0).is_err());
    }

    #[test]
    fn random_generate_list() {
        let mut rng = rand::thread_rng();

        let size: usize = 100;
        let test_iter: usize = 10;

        for _ in 0..test_iter {
            let mut random_data: Vec<Vec<f32>> =
                (0..size).map(|_| vec![rng.gen::<f32>()]).collect();
            let k = rng.gen_range(1..size + 1);

            // kth_smallest result
            let cut_axis = 0;
            let res_idx = kth_smallest(&mut random_data, k, cut_axis).unwrap();
            check(&random_data, res_idx, cut_axis);

            // sort results
            random_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
            assert_eq!(random_data[res_idx], random_data[k - 1]);
        }
    }
}
