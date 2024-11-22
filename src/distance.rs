use crate::data::Object;
use ndarray::Array2;

// Функция для расчета расстояния между двумя объектами
pub fn calculate_distance(obj1: &Object, obj2: &Object) -> f64 {
    obj1.features
        .iter()
        .zip(&obj2.features)
        .map(|(x1, x2)| (x1 - x2).powi(2))
        .sum::<f64>()
        .sqrt()
}

// Построение матрицы расстояний
pub fn build_distance_matrix(data: &Vec<Object>) -> Array2<f64> {
    let size = data.len();
    let mut matrix = Array2::<f64>::zeros((size, size));

    for i in 0..size {
        for j in 0..size {
            if i != j {
                matrix[[i, j]] = calculate_distance(&data[i], &data[j]);
            }
        }
    }
    matrix
}
