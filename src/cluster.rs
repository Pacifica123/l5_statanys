use crate::{data::{ClusterMerge, Object}, distance::{calculate_cluster_distance, calculate_distance}, output::format_clusters};


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
    let mut iteration: i32 = 0;

    while clusters.len() > 1 {
        let mut min_distance = f64::MAX;
        let mut merge_indices = (0, 0);

        // Найти ближайшие кластеры
        for i in 0..clusters.len() {
            for j in (i + 1)..clusters.len() {
                let dist = calculate_cluster_distance(&clusters[i], &clusters[j]); // Новый метод
                if dist < min_distance {
                    min_distance = dist;
                    merge_indices = (i, j);
                }
            }
        }

        // Объединить кластеры
        let cluster_j = clusters.remove(merge_indices.1);
        clusters[merge_indices.0].extend(cluster_j);

        // Вывод информации
        if iteration%10 == 0
        { println!(
                "Шаг {}: Расстояние (min) = {:.6} | Кластеры: {}",
                iteration,
                min_distance,
                format_clusters(&clusters)
            );}
        iteration += 1;
    }
}

// Предварительное вычисление расстояний
pub fn hierarchical_clustering_optimized(data: Vec<Object>) {
    let n = data.len();
    let mut clusters: Vec<Vec<Object>> = data.into_iter().map(|obj| vec![obj]).collect();
    let mut distances = vec![vec![0.0; n]; n];

    // Предварительное вычисление расстояний
    for i in 0..n {
        for j in i + 1..n {
            distances[i][j] = calculate_distance(&clusters[i][0], &clusters[j][0]);
            distances[j][i] = distances[i][j];
        }
    }

    while clusters.len() > 1 {
        // Найти минимальное расстояние
        let mut min_distance = f64::MAX;
        let mut merge_indices = (0, 0);

        for i in 0..clusters.len() {
            for j in i + 1..clusters.len() {
                if distances[i][j] < min_distance {
                    min_distance = distances[i][j];
                    merge_indices = (i, j);
                }
            }
        }

        // Объединение кластеров
        let cluster_j = clusters.remove(merge_indices.1);
        clusters[merge_indices.0].extend(cluster_j);

        // Обновление расстояний
        for i in 0..clusters.len() {
            distances[merge_indices.0][i] = calculate_cluster_distance(&clusters[merge_indices.0], &clusters[i]);
            distances[i][merge_indices.0] = distances[merge_indices.0][i];
        }
    }
}

pub fn hierarchical_clustering_with_trace(data: Vec<Object>) -> Vec<ClusterMerge> {
    let n = data.len();
    let mut clusters: Vec<Vec<Object>> = data.into_iter().map(|obj| vec![obj]).collect();
    let mut distances = vec![vec![0.0; n]; n];
    let mut merge_trace: Vec<ClusterMerge> = Vec::new();

    // Предварительное вычисление расстояний
    for i in 0..n {
        for j in i + 1..n {
            distances[i][j] = calculate_distance(&clusters[i][0], &clusters[j][0]);
            distances[j][i] = distances[i][j];
        }
    }

    while clusters.len() > 1 {
        // Найти минимальное расстояние
        let mut min_distance = f64::MAX;
        let mut merge_indices = (0, 0);

        for i in 0..clusters.len() {
            for j in i + 1..clusters.len() {
                if distances[i][j] < min_distance {
                    min_distance = distances[i][j];
                    merge_indices = (i, j);
                }
            }
        }

        // Записать слияние
        merge_trace.push(ClusterMerge {
            cluster1: merge_indices.0,
            cluster2: merge_indices.1,
            distance: min_distance,
        });

        // Объединение кластеров
        let cluster_j = clusters.remove(merge_indices.1);
        clusters[merge_indices.0].extend(cluster_j);

        // Обновление расстояний
        for i in 0..clusters.len() {
            distances[merge_indices.0][i] = calculate_cluster_distance(&clusters[merge_indices.0], &clusters[i]);
            distances[i][merge_indices.0] = distances[merge_indices.0][i];
        }
    }

    merge_trace
}