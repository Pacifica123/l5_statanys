use csv::Writer;
use ndarray::Array2;
use std::error::Error;
use crate::data::{Dataset, Object};

/// Порог количества записей для вывода в терминал
const TERMINAL_OUTPUT_THRESHOLD: usize = 50;

/// Функция вывода данных: в терминал или CSV
pub fn output_dataset(dataset: &Dataset, file_path: &str) -> Result<(), Box<dyn Error>> {
    if dataset.objects.len() <= TERMINAL_OUTPUT_THRESHOLD {
        // Если данных мало, выводим в терминал
        println!("Данные:\n{:?}", dataset);
    } else {
        // Если данных много, записываем в CSV
        let mut writer = Writer::from_path(file_path)?;
        writer.write_record(&dataset.objects[0].feature_names)?;

        for object in &dataset.objects {
            let record: Vec<String> = object
                .features
                .iter()
                .map(|&f| f.to_string())
                .collect();
            writer.write_record(&record)?;
        }

        writer.flush()?;
        println!("Данные записаны в файл: {}", file_path);
    }

    Ok(())
}

/// Функция вывода матрицы расстояний: в терминал или CSV
pub fn output_distance_matrix(
    matrix: &Array2<f64>,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let rows = matrix.nrows();
    let cols = matrix.ncols();

    if rows <= TERMINAL_OUTPUT_THRESHOLD {
        // Если данных мало, выводим в терминал
        println!("Матрица расстояний:");
        for row in matrix.genrows() {
            println!("{:?}", row.to_vec());
        }
    } else {
        // Если данных много, записываем в CSV
        let mut writer = Writer::from_path(file_path)?;

        for row in matrix.genrows() {
            let record: Vec<String> = row.iter().map(|&d| d.to_string()).collect();
            writer.write_record(&record)?;
        }

        writer.flush()?;
        println!("\nМатрица расстояний записана в файл: {}", file_path);
    }

    Ok(())
}

// Форматирование кластеров для вывода
pub fn format_clusters(clusters: &[Vec<Object>]) -> String {
    let cluster_strings: Vec<String> = clusters
        .iter()
        .enumerate()
        .map(|(i, cluster)| {
            let ids: Vec<usize> = cluster.iter().map(|obj| obj.id).collect();
            let formatted_ids = format_large_list(&ids);
            format!("[{}]: {}", i + 1, formatted_ids)
        })
        .collect();

    if clusters.len() > 10 {
        let head = &cluster_strings[..3];
        let tail = &cluster_strings[clusters.len() - 3..];
        format!(
            "{} <...> {}",
            head.join(" | "),
            tail.join(" | ")
        )
    } else {
        cluster_strings.join(" | ")
    }
}

// Форматирование кластеров для вывода
pub fn format_clusters_full(clusters: &[Vec<Object>]) -> String {
    let cluster_strings: Vec<String> = clusters
        .iter()
        .enumerate()
        .map(|(i, cluster)| {
            let ids: Vec<usize> = cluster.iter().map(|obj| obj.id).collect();
            format!("[{}]: {:?}", i + 1, ids) // Используем {:?} для вывода всех ID
        })
        .collect();

    cluster_strings.join(" | ") // Возвращаем все кластеры без сокращений
}

// Форматирование длинных списков (внутри кластеров)
fn format_large_list(ids: &[usize]) -> String {
    if ids.len() > 10 {
        let head = &ids[..3];
        let tail = &ids[ids.len() - 3..];
        format!(
            "{:?} <...> {:?}",
            head,
            tail
        )
    } else {
        format!("{:?}", ids)
    }
}