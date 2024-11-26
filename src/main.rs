mod data;
mod distance;
mod cluster;
mod dendrogram;
mod csv_parser;
mod output;

use csv_parser::parse_csv;
use data::ClusterMerge;
use dendrogram::{plot_dendrogram, plot_dendrogram2, plot_dendrogram_to_png};
use distance::build_distance_matrix;
use cluster::{hierarchical_clustering, hierarchical_clustering_optimized, hierarchical_clustering_with_trace};

fn main() {
    // Пример данных
    match parse_csv("diamonds.csv", b',') {
        Ok(dataset) => {
            println!("Набор данных успешно загружен и обработан:");
            output::output_dataset(&dataset, "filtered.csv")
                .unwrap_or_else(|e| println!("Ошибка при выводе данных: {}", e));

            // Нормализация
            let normalized_dataset = dataset.normalize();
            output::output_dataset(&normalized_dataset, "normalized_dataset.csv")
                .unwrap_or_else(|e| println!("Ошибка при выводе данных: {}", e));

            // Матрица расстояний           
            let distance_matrix = build_distance_matrix(&normalized_dataset.objects);
            output::output_distance_matrix(&distance_matrix, "distance_matrix.csv")
                .unwrap_or_else(|e| println!("Ошибка при выводе матрицы расстояний: {}", e));

            // Иерархическая кластеризация
            // hierarchical_clustering(normalized_dataset.objects);
            hierarchical_clustering_optimized(&normalized_dataset.objects);
            let clusters = hierarchical_clustering_with_trace(normalized_dataset.objects);

            plot_dendrogram(&clusters);

            let trace_test = vec![
                ClusterMerge {
                    cluster1: 0,
                    cluster2: 1,
                    distance: 0.7,
                },
                ClusterMerge {
                    cluster1: 2,
                    cluster2: 3,
                    distance: 1.1,
                },
                ClusterMerge {
                    cluster1: 4,
                    cluster2: 5,
                    distance: 1.4,
                },
            ];              
            // plot_dendrogram_to_png(&clusters, "final.png");
        }
        Err(err) => {
            println!("Ошибка при обработке CSV-файла: {}", err);
        }
    };
}
