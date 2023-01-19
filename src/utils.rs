
// TODO: rename fn
pub(crate) fn quick_select<T: std::cmp::PartialEq + std::cmp::PartialOrd + Copy>(
    data: &Vec<T>,
    k: usize,
) -> Result<T, String> {
    if k == 0 {
        return Err("k should be greater than 0, but you give 0.".to_string());
    }

    let size = data.len();
    if size < k {
        return Err(format!("the size of data [{size}] is less than k [{k}]"));
    }

    let mut indices: Vec<usize> = (0..size).collect();

    Ok(_quick_select(data, &mut indices, 0, size - 1, k))
}

fn _quick_select<T: std::cmp::PartialEq + std::cmp::PartialOrd + Copy>(
    data: &Vec<T>,
    indices: &mut Vec<usize>,
    l: usize,
    r: usize,
    k: usize,
) -> T {
    if l >= r {
        return data[indices[l]];
    }

    let mut i = l as i32 - 1;
    let mut j = r as i32 + 1;
    // let x = data[indices[(l + r) >> 1]];
    let x = data[indices[l]];

    while i < j {
        loop {
            i += 1;
            if data[indices[i as usize]] >= x {
                break;
            }
        }
        loop {
            j -= 1;
            if data[indices[j as usize]] <= x {
                break;
            }
        }
        if i < j {
            indices.swap(i as usize, j as usize);
        }
    }

    if k <= j as usize - l + 1 {
        return _quick_select(data, indices, l, j as usize, k);
    }
    _quick_select(data, indices, j as usize + 1, r, k - (j as usize - l + 1))
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn basic() {
        let a = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        assert_eq!(quick_select(&a, 3), Ok(3));
    }

    #[test]
    fn list_size_less_than_k() {
        let a = Vec::<i32>::new();
        assert_eq!(quick_select(&a, 3), Err("the size of data [0] is less than k [3]".to_string()));

        let a = vec![1,2,3,4,5];
        // let res = quick_select(&a, 3).map_err(|e| e.kind());
        assert_eq!(quick_select(&a, 10), Err("the size of data [5] is less than k [10]".to_string()));
    }

    #[test]
    fn zero_k() {
        let a = Vec::<i32>::new();
        assert_eq!(quick_select(&a, 0), Err("k should be greater than 0, but you give 0.".to_string()));

        let a = vec![1,2,3,4,5];
        // let res = quick_select(&a, 3).map_err(|e| e.kind());
        assert_eq!(quick_select(&a, 0), Err("k should be greater than 0, but you give 0.".to_string()));
    }

    #[test]
    fn pre_sorted() {
        let a = vec![1,2,3,4,5];
        // let res = quick_select(&a, 3).map_err(|e| e.kind());
        assert_eq!(quick_select(&a, 2), Ok(2));
    }

    #[test]
    fn random_generate_list() {
        let mut rng = rand::thread_rng();

        let size: usize = 100;
        let test_iter: usize = 10;

        for _ in 0..test_iter {
            let mut random_data: Vec<f32> = (0..size).map(|_| rng.gen::<f32>()).collect();
            let k = rng.gen_range(1..size + 1);

            // quick select result
            let qs_res = quick_select(&random_data, k).unwrap();

            // sort results
            random_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
            assert_eq!(qs_res, random_data[k - 1]);
        }
    }
}

