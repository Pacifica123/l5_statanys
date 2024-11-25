use crate::data::Object;
use ndarray::Array2;
// для оптимизированного поиска расстояний
use rayon::prelude::*;
use ndarray::s;
use indicatif::{ProgressBar, ProgressStyle};

// Функция для расчета расстояния между двумя объектами
pub fn calculate_distance(obj1: &Object, obj2: &Object) -> f64 {
    obj1.features
        .iter()
        .zip(&obj2.features)
        .map(|(x1, x2)| (x1 - x2).powi(2))
        .sum::<f64>()
        .sqrt()
}

// Рассчитать расстояние между кластерами (средняя связь)
pub fn calculate_cluster_distance(cluster_a: &[Object], cluster_b: &[Object]) -> f64 {
    let mut total_distance = 0.0;
    let mut count = 0;

    for obj_a in cluster_a {
        for obj_b in cluster_b {
            total_distance += calculate_distance(obj_a, obj_b);
            count += 1;
        }
    }

    total_distance / count as f64
}

// оптимизировано через rayon
pub fn build_distance_matrix(data: &[Object]) -> Array2<f64> {
    let size = data.len();
    let mut matrix = Array2::<f64>::zeros((size, size));

    // Используем парные индексы для симметричного расчета
    let indices: Vec<(usize, usize)> = (0..size)
        .flat_map(|i| (i + 1..size).map(move |j| (i, j)))
        .collect();
    let total_pairs = indices.len() as u64;

    let pb = ProgressBar::new(total_pairs);
        // Настройка стиля прогресс-бара
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%)")
        .unwrap() // Обработка Result
        .progress_chars("=>:");

    pb.set_style(style);

        for (i, j) in indices {
            let distance = calculate_distance(&data[i], &data[j]);
            matrix[[i, j]] = distance;
            matrix[[j, i]] = distance;
    
            pb.inc(1);

        }
        pb.finish_with_message("Расчет завершен");
    
    matrix
}

// С треугольной симметрией матрицы
// pub fn build_distance_matrix(data: &[Object]) -> Array2<f64> {
//     let size = data.len();
//     let mut matrix = Array2::<f64>::zeros((size, size));

//     for i in 0..size {
//         for j in (i + 1)..size {
//             let distance = calculate_distance(&data[i], &data[j]);
//             matrix[[i, j]] = distance;
//             matrix[[j, i]] = distance; // Используем симметрию
//         }
//         if i%100 == 0{
//             println!("обработано {} объектов", i);
//         }
        
//     }
//     matrix
// }
// // Построение матрицы расстояний
// pub fn build_distance_matrix(data: &Vec<Object>) -> Array2<f64> {
//     let size = data.len();
//     let mut matrix = Array2::<f64>::zeros((size, size));

//     for i in 0..size {
//         for j in 0..size {
//             if i != j {
//                 matrix[[i, j]] = calculate_distance(&data[i], &data[j]);
//             }
//         }
//     }
//     matrix
// }
