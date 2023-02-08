use kdtree::KDTree;
use rand::Rng;

fn random_points(length: usize) -> Vec<Vec<f32>> {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| vec![rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()])
        .collect()
}

fn main() {
    let points = random_points(500);
    let query_point = &random_points(1)[0];
    let k = 10;

    let addition_points = random_points(200);

    // kdtree addition
    let mut tree = KDTree::new(&points).unwrap();
    for p in addition_points.iter() {
        tree.add(p);
    }

    println!("{:?}", tree.search_knn(query_point, k).unwrap());
}
