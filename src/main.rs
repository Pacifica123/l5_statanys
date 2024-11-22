mod data;
mod distance;
mod cluster;
mod dendrogram;

use data::{Dataset, Object};
use distance::build_distance_matrix;
use cluster::hierarchical_clustering;

fn main() {
    // Пример данных
    let data = vec![
        Object {
            id: 1,
            features: vec![1.0, 2.0],
            feature_names: vec!["Возраст".to_string(), "Вес".to_string()],
        },
        Object {
            id: 2,
            features: vec![2.0, 3.0],
            feature_names: vec!["Возраст".to_string(), "Вес".to_string()],
        },
        Object {
            id: 3,
            features: vec![5.0, 6.0],
            feature_names: vec!["Возраст".to_string(), "Вес".to_string()],
        },
    ];  

    // Создание набора данных
    let dataset = Dataset::new(data).expect("Ошибка валидации данных");
    println!("Исходный набор данных: {:?}", dataset);

    // Нормализация
    let normalized_dataset = dataset.normalize();
    println!("Нормализованный набор данных: {:?}", normalized_dataset);

    // Матрица расстояний
    let distance_matrix = build_distance_matrix(&normalized_dataset.objects);
    println!("Матрица расстояний:\n{:?}", distance_matrix);

    // Иерархическая кластеризация
    hierarchical_clustering(normalized_dataset.objects);
}
