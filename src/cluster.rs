use crate::data::Object;

// Центр кластера (геометрическое среднее)
pub fn cluster_center(cluster: &Vec<Object>) -> Vec<f64> {
    let num_features = cluster[0].features.len();
    (0..num_features)
        .map(|i| {
            let product = cluster.iter().map(|obj| obj.features[i]).product::<f64>();
            product.powf(1.0 / cluster.len() as f64)
        })
        .collect()
}

// Итеративное объединение кластеров
pub fn  hierarchical_clustering(data: Vec<Object>) {
    let mut clusters: Vec<Vec<Object>> = data.into_iter().map(|obj| vec![obj]).collect();
    let mut iteration = 0;

    while clusters.len() > 1 {
        let mut min_distance = f64::MAX;
        let mut merge_indices = (0, 0);

        // Найти ближайшие кластеры
        for i in 0..clusters.len() {
            for j in (i + 1)..clusters.len() {
                let dist = crate::distance::calculate_distance(
                    &clusters[i][0], // Используем первый объект как представитель кластера
                    &clusters[j][0],
                );
                if dist < min_distance {
                    min_distance = dist;
                    merge_indices = (i, j);
                }
            }
        }

        // Объединить кластеры
        let cluster_j = clusters.remove(merge_indices.1);
        clusters[merge_indices.0].extend(cluster_j);

        // Вывод информации о итерации
        println!(
            "Шаг {}: Расстояние (min) между объединенными кластерами {}",
            iteration, min_distance
        );
        iteration += 1;
    }
}
