use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kdtree::KDTree;
use rand::Rng;

fn random_points(length: usize) -> Vec<Vec<f32>> {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| vec![rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()])
        .collect()
}

fn build_with_1k_3d_points(c: &mut Criterion) {
    let points = random_points(1000);
    c.bench_function("build_with_1k_3d_points", |b| b.iter(|| KDTree::new(black_box(&points))));
}

fn add_to_kdtree_with_1k_3d_points(c: &mut Criterion) {
    let points = random_points(1000);
    let add_point = &random_points(1)[0];
    let mut tree = KDTree::new(&points).unwrap();
    c.bench_function("add_to_kdtree_with_1k_3d_points", |b| b.iter(|| tree.add(add_point)));
}

fn nn1_with_1k_3d_points(c: &mut Criterion) {
    let points = random_points(1000);
    let query_point = &random_points(1)[0];
    let tree = KDTree::new(&points).unwrap();
    c.bench_function("nn1_with_1k_3d_points", |b| b.iter(|| tree.search_knn(black_box(query_point), black_box(1))));
}

fn nn10_with_1k_3d_points(c: &mut Criterion) {
    let points = random_points(1000);
    let query_point = &random_points(1)[0];
    let tree = KDTree::new(&points).unwrap();
    c.bench_function("nn10_with_1k_3d_points", |b| b.iter(|| tree.search_knn(black_box(query_point), black_box(10))));
}

criterion_group!(benches, build_with_1k_3d_points, add_to_kdtree_with_1k_3d_points, nn1_with_1k_3d_points, nn10_with_1k_3d_points);
criterion_main!(benches);
