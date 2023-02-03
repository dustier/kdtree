use crate::utils;
use std::cmp::{PartialEq, PartialOrd};

pub enum Error {
    EmptyInput,
}

// TODO: data generics

#[derive(Default)]
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
        match Self::build_tree(&mut points_, 0, 0, points.len() - 1) {
            Some(node) => Ok(node),
            None => Err(Error::EmptyInput),
        }
    }

    fn build_tree(points: &mut [Vec<T>], current_cut_axis: usize, l: usize, r: usize) -> Option<Self> {
        if l > r {
            return None;
        }

        let dim = points[l].len();
        let pivot = utils::kth_smallest(points, (l + r) / 2, current_cut_axis).unwrap();

        let left = Self::build_tree(points, (current_cut_axis + 1) % dim, l, pivot - 1).map(Box::new);
        let right = Self::build_tree(points, (current_cut_axis + 1) % dim, pivot + 1, r).map(Box::new);

        Some(Self {
            data: points[pivot].clone(),

            cut_axis: current_cut_axis,
            left,
            right,
        })
    }

    pub fn search_knn(&self, k: usize) -> Result<Vec<T>, Error> {
        unimplemented!();
    }

    pub fn add(&mut self, points: T) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn remove(&mut self, points: T) -> Result<(), Error> {
        unimplemented!();
    }

}
