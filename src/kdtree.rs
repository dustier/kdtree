// use crate::utils;

pub enum Error {

}

// TODO: data generics
pub struct KDTree<T: std::cmp::PartialEq> {
    data: Vec<T>,
    data_dim: usize,
    cut_axis: usize,

    left: Option<Box<KDTree<T>>>,
    right: Option<Box<KDTree<T>>>,
}

// impl<T: std::cmp::PartialEq> KDTree<T> {
//     pub fn new(points: &Vec<T>) -> Self {
//     }

//     pub fn search_knn(k: usize) -> Result<Vec<T>, Error> {

//     }

//     pub fn add(points: T) -> Result<(), Error> {

//     }

// }
