use kdtree::KDTree;
use rand::Rng;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

#[derive(Debug, PartialEq)]
struct Element {
    distance: f32,
    point: Vec<f32>,
}

impl Eq for Element {}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn random_points(length: usize) -> Vec<Vec<f32>> {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| vec![rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()])
        .collect()
}

fn calculate_distance(p1: &[f32], p2: &[f32]) -> f32 {
    let mut dist = 0.0_f32;
    for (i, p) in p1.iter().enumerate() {
        dist += (p - p2[i]) * (p - p2[i]);
    }
    dist
}

fn brute_force_search(points: &[Vec<f32>], query_point: &[f32], k: usize) -> Vec<Vec<f32>> {
    let mut elements: Vec<Element> = points
        .iter()
        .map(|p| {
            let distance = calculate_distance(p, query_point);
            Element {
                distance,
                point: p.clone(),
            }
        })
        .collect();
    elements.sort();

    elements
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i < &k)
        .map(|(_, e)| e.point)
        .collect()
}

fn basic() -> bool {
    let points = random_points(1000);
    let query_point = &random_points(1)[0];
    let k = 10;
    let ans = brute_force_search(&points, query_point, k);

    let tree = KDTree::new(&points).unwrap();
    tree.search_knn(query_point, k).unwrap() == ans
}

fn with_addition() -> bool {
    let mut points = random_points(500);
    let query_point = &random_points(1)[0];
    let addition_points = random_points(200);
    let k = 10;

    // kdtree addition
    let mut tree = KDTree::new(&points).unwrap();
    for p in addition_points.iter() {
        tree.add(p);
    }

    points.extend_from_slice(&addition_points);
    let ans = brute_force_search(&points, query_point, k);

    tree.search_knn(query_point, k).unwrap() == ans
}

#[test]
fn iter_kdtree_works() {
    for _ in 0..50 {
        assert!(basic());
        assert!(with_addition());
    }
}
