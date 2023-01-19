
pub(crate) fn quick_select<T: std::cmp::PartialEq + std::cmp::PartialOrd + Copy>(
    data: &Vec<T>,
    indices: &mut Vec<usize>,
    l: usize,
    r: usize,
    k: usize,
) -> T {
    if l >= r {
        return data[indices[l]];
    }

    let mut i = l - 1;
    let mut j = r + 1;
    let x = &data[indices[(l + r) >> 1]];

    while i < j {
        loop {
            i += 1;
            if data[indices[i]] >= *x {
                break;
            }
        }
        loop {
            j -= 1;
            if data[indices[j]] <= *x {
                break;
            }
        }
    }

    if k <= j - l + 1 {
        return quick_select(data, indices, l, j, k);
    }
    quick_select(data, indices, j + 1, r, k - (j - l + 1))
}

#[test]
fn test_quick_select() {
}
