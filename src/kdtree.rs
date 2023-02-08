use crate::utils::{calculate_distance, kth_smallest, Element};
use std::cmp::{PartialEq, PartialOrd};
use std::collections::BinaryHeap;

#[derive(Debug)]
pub enum Error {
    EmptyInput,

    InvalidK,
    // PointDoesNotExist,
}

// TODO: data generics

#[derive(Default, PartialEq, Debug)]
pub struct KDTree<T: PartialOrd + Copy + Clone + Into<f64>> {
    data: Vec<T>,
    cut_axis: usize,

    left: Option<Box<KDTree<T>>>,
    right: Option<Box<KDTree<T>>>,
}

impl<T> KDTree<T>
where
    T: PartialOrd + Copy + Clone + Into<f64> + std::fmt::Debug,
{
    // public function
    pub fn new(points: &[Vec<T>]) -> Result<Self, Error> {
        if points.is_empty() {
            return Err(Error::EmptyInput);
        }

        // TODO: use variance to decide initial cut_axis
        let mut points_ = points.to_vec();
        Ok(Self::build_tree(&mut points_, 0).unwrap())
    }

    pub fn search_knn(&self, point: &[T], k: usize) -> Result<Vec<Vec<T>>, Error> {
        if k < 1 {
            return Err(Error::InvalidK);
        }

        let mut heap = BinaryHeap::new();
        self._search_knn(point, k, &mut heap);

        // Ok(heap.into_iter().collect())
        // Ok(heap.into_iter().map(|e| e.point).collect())
        assert!(heap.len() == k);
        let mut res: Vec<Vec<T>> = Vec::new();
        while !heap.is_empty() {
            res.push(heap.pop().unwrap().point);
        }
        res.reverse();
        Ok(res)
    }

    pub fn add(&mut self, point: &Vec<T>) {
        // TODO: invalid input point

        if point[self.cut_axis] < self.data[self.cut_axis] {
            match self.left {
                Some(ref mut node) => node.add(point),
                None => {
                    self.left = Some(Box::new(Self {
                        data: point.clone(),
                        cut_axis: (self.cut_axis + 1) % point.len(),
                        left: None,
                        right: None,
                    }))
                }
            }
        } else {
            match self.right {
                Some(ref mut node) => node.add(point),
                None => {
                    self.right = Some(Box::new(Self {
                        data: point.clone(),
                        cut_axis: (self.cut_axis + 1) % point.len(),
                        left: None,
                        right: None,
                    }))
                }
            }
        }
    }

    // TODO: remove
    // pub fn remove(&mut self, point: &Vec<T>) -> Result<Option<Self>, Error> {
    //     self._remove(point, None)?;
    //     Ok(())
    // }

    // ===== private function =====
    fn build_tree(
        points: &mut [Vec<T>],
        current_cut_axis: usize,
        // l: isize,
        // r: isize,
    ) -> Option<Self> {
        if points.is_empty() {
            return None;
        }
        if points.len() == 1 {
            return Some(Self {
                data: points[0].clone(),
                cut_axis: current_cut_axis,

                left: None,
                right: None,
            });
        }

        let dim = points[0].len();
        let k = points.len() / 2 + 1;
        let pivot = kth_smallest(points, k, current_cut_axis).unwrap();

        let root_data = points[pivot].clone();

        let left =
            Self::build_tree(&mut points[..pivot], (current_cut_axis + 1) % dim).map(Box::new);
        let right =
            Self::build_tree(&mut points[pivot + 1..], (current_cut_axis + 1) % dim).map(Box::new);

        Some(Self {
            data: root_data,
            cut_axis: current_cut_axis,

            left,
            right,
        })
    }

    fn _search_knn(&self, point: &[T], k: usize, heap: &mut BinaryHeap<Element<T>>) {
        let distance = calculate_distance(&self.data, point);
        let element = Element {
            distance,
            point: self.data.clone(),
        };

        if heap.len() < k {
            heap.push(element);
        } else if &element < heap.peek().unwrap() {
            heap.pop();
            heap.push(element);
        }

        let go_left = point[self.cut_axis] < self.data[self.cut_axis];

        if go_left {
            if let Some(node) = &self.left {
                node._search_knn(point, k, heap);
            }
        } else if let Some(node) = &self.right {
            node._search_knn(point, k, heap);
        }

        let dist_to_cut_plane: f64 = point[self.cut_axis].into() - self.data[self.cut_axis].into();
        let dist_to_cut_plane_square = dist_to_cut_plane * dist_to_cut_plane;

        if heap.len() < k || dist_to_cut_plane_square < heap.peek().unwrap().distance {
            if go_left {
                if let Some(node) = &self.right {
                    node._search_knn(point, k, heap);
                }
            } else if let Some(node) = &self.left {
                node._search_knn(point, k, heap);
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn basic() {
        let points = [vec![1], vec![2], vec![2], vec![3]];
        let tree = KDTree::new(&points).unwrap();

        let left = KDTree {
            data: vec![1],
            cut_axis: 0,
            left: None,
            right: None,
        };
        let right = KDTree {
            data: vec![3],
            cut_axis: 0,
            left: Some(Box::new(KDTree {
                data: vec![2],
                cut_axis: 0,
                left: None,
                right: None,
            })),
            right: None,
        };

        let ans = KDTree {
            data: vec![2],
            cut_axis: 0,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        assert_eq!(tree, ans);

        // add
        let points = [vec![2], vec![2], vec![3]];
        let mut tree = KDTree::new(&points).unwrap();
        tree.add(&vec![1]);

        assert_eq!(tree, ans);

        // search
        let query_point = vec![4];
        let res = tree.search_knn(&query_point, 2).unwrap();
        let ans = vec![vec![3], vec![2]];
        assert_eq!(res, ans);
    }

    #[test]
    fn search() {
        let points = [
            vec![0.35010368, 0.6240131, 0.5635964],
            vec![0.27942437, 0.8336378, 0.8208322],
            vec![0.19197953, 0.049370766, 0.46151704],
            vec![0.1565113, 0.5717637, 0.31621546],
            vec![0.64521194, 0.52954865, 0.77038324],
        ];
        let query_point = vec![0.23113745, 0.29625928, 0.031125069];
        let k = 2;

        let ans = vec![
            vec![0.1565113, 0.5717637, 0.31621546],
            vec![0.19197953, 0.049370766, 0.46151704],
        ];

        let tree = KDTree::new(&points).unwrap();
        let res = tree.search_knn(&query_point, k).unwrap();
        assert_eq!(res, ans);
    }
}
