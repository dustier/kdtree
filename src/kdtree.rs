use crate::utils;
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
pub struct KDTree<T: Ord + Clone> {
    data: Vec<T>,
    cut_axis: usize,

    left: Option<Box<KDTree<T>>>,
    right: Option<Box<KDTree<T>>>,
}

impl<T: Ord + Clone> KDTree<T> {
    // public function
    pub fn new(points: &[Vec<T>]) -> Result<Self, Error> {
        if points.is_empty() {
            return Err(Error::EmptyInput);
        }

        // TODO: use variance to decide initial cut_axis
        let mut points_ = points.to_vec();
        Ok(Self::build_tree(&mut points_, 0, 0, points.len() as isize - 1).unwrap())
    }

    pub fn search_knn(&self, point: &Vec<T>, k: usize) -> Result<Vec<Vec<T>>, Error> {
        if k < 1 {
            return Err(Error::InvalidK);
        }

        let mut heap = BinaryHeap::new();
        self._search_knn(point, k, &mut heap);

        Ok(heap.into_iter().collect())
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
        l: isize,
        r: isize,
    ) -> Option<Self> {
        if l > r {
            return None;
        }

        let dim = points[l as usize].len();
        let k = ((l + r) / 2 + 1) as usize;
        let pivot = utils::kth_smallest(points, k, current_cut_axis).unwrap() as isize;
        let pivot = std::cmp::max(pivot, l);

        let left =
            Self::build_tree(points, (current_cut_axis + 1) % dim, l, pivot - 1).map(Box::new);
        let right =
            Self::build_tree(points, (current_cut_axis + 1) % dim, pivot + 1, r).map(Box::new);

        Some(Self {
            data: points[pivot as usize].clone(),
            cut_axis: current_cut_axis,

            left,
            right,
        })
    }

    fn _search_knn(&self, point: &Vec<T>, k: usize, heap: &mut BinaryHeap<Vec<T>>) {

    }

    // fn _remove(&mut self, point: &Vec<T>, father: Option<&mut Self>) -> Result<&Self, Error> {

    // }

    // fn find_min(&self, dim: usize) -> Vec<T> {

    //     if dim == self.cut_axis {
    //         match self.left {
    //             Some(ref node) => node.find_min(dim),
    //             None => self.data.clone(),
    //         }
    //     } else {
    //         let mut res = self.data.clone();
    //         if let Some(ref node) = self.left {
    //             if node.data[dim] < res[dim] {
    //                 res = node.data.clone();
    //             }
    //         }
    //         if let Some(ref node) = self.right {
    //             if node.data[dim] < res[dim] {
    //                 res = node.data.clone();
    //             }
    //         }
    //         res
    //     }
    // }
}

mod test {
    use super::*;

    #[test]
    fn build_basic() {
        let points = [vec![1], vec![2], vec![2], vec![3]];
        let tree = KDTree::new(&points).unwrap();

        let left = KDTree {
            data: vec![1],
            cut_axis: 0,
            left: None,
            right: None,
        };
        let right = KDTree {
            data: vec![2],
            cut_axis: 0,
            left: None,
            right: Some(Box::new(KDTree {
                data: vec![3],
                cut_axis: 0,
                left: None,
                right: None,
            })),
        };

        let ans = KDTree {
            data: vec![2],
            cut_axis: 0,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        assert_eq!(tree, ans);

        // add
        let points = [vec![1], vec![2], vec![2]];
        let mut tree = KDTree::new(&points).unwrap();
        tree.add(&vec![3]);

        assert_eq!(tree, ans);
    }
}
