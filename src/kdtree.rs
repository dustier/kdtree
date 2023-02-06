use crate::utils;
use std::cmp::{PartialEq, PartialOrd};

#[derive(Debug)]
pub enum Error {
    EmptyInput,
}

// TODO: data generics

#[derive(Default, PartialEq, Debug)]
pub struct KDTree<T: PartialOrd + Clone> {
    data: Vec<T>,
    // data_dim: usize,
    cut_axis: usize,

    left: Option<Box<KDTree<T>>>,
    right: Option<Box<KDTree<T>>>,
}

impl<T: PartialOrd + Clone> KDTree<T> {
    pub fn new(points: &[Vec<T>]) -> Result<Self,Error> {

        if points.is_empty() {
            return Err(Error::EmptyInput);
        }

        // TODO: use variance to decide initial cut_axis
        let mut points_ = points.to_vec();
        match Self::build_tree(&mut points_, 0, 0, points.len() as isize - 1) {
            Some(node) => Ok(node),
            None => Err(Error::EmptyInput),
        }
    }

    fn build_tree(points: &mut [Vec<T>], current_cut_axis: usize, l: isize, r: isize) -> Option<Self> {
        if l > r {
            return None;
        }

        let dim = points[l as usize].len();
        let k = ((l + r) / 2 + 1) as usize;
        let pivot = utils::kth_smallest(points, k, current_cut_axis).unwrap() as isize;
        let pivot = std::cmp::max(pivot, l);

        let left = Self::build_tree(points, (current_cut_axis + 1) % dim, l, pivot - 1).map(Box::new);
        let right = Self::build_tree(points, (current_cut_axis + 1) % dim, pivot + 1, r).map(Box::new);

        Some(Self {
            data: points[pivot as usize].clone(),

            cut_axis: current_cut_axis,
            left,
            right,
        })
    }

    pub fn search_knn(&self, k: usize) -> Result<Vec<T>, Error> {
        unimplemented!();

    }

    pub fn add(&mut self, point: &Vec<T>) -> Result<(), Error> {
        let dim = point.len();
        let son = if point[self.cut_axis] < self.data[self.cut_axis] {
            &mut self.left
        } else {
            &mut self.right
        };

        match son {
            Some(node) => node.add(point),
            None => {
                self.right = Some(Box::new(KDTree{
                    data: point.clone(),
                    cut_axis: (self.cut_axis + 1) % dim,
                    left: None,
                    right: None,
                }));
                Ok(())
            }
        }
    }

    pub fn remove(&mut self, point: &Vec<T>) -> Result<(), Error> {
        unimplemented!();
    }
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
            right: Some(Box::new(KDTree{
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
        tree.add(&vec![3]).unwrap();

        assert_eq!(tree, ans);
    }
}
