use csv::ReaderBuilder;
use std::error::Error;

use crate::data::{Dataset, Object};

pub fn parse_csv(
    file_path: &str,
    delimiter: u8,
) -> Result<Dataset, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(true)
        .from_path(file_path)?;

    let headers = reader.headers()?.iter().map(|h| h.trim().to_string()).collect::<Vec<_>>();

    // Сначала собираем все записи, чтобы проверить их на числовые значения
    let mut objects = vec![];
    let mut numeric_headers = vec![false; headers.len()]; // Массив для отслеживания числовых заголовков

    let mut total_cells = 0; // Общее количество ячеек
    const MAX_CELLS: usize = 1_000; // Максимальное количество ячеек

    for (id, record) in reader.records().enumerate() {
        let record = record?;
        let mut features = vec![];

        for (i, field) in record.iter().enumerate() {
            if let Ok(value) = field.parse::<f64>() {
                features.push(value);
                numeric_headers[i] = true; // Устанавливаем флаг для числового заголовка
            }
        }

        if !features.is_empty() {
            objects.push(Object {
                id: id + 1,
                features: features.clone(),
                feature_names: headers.iter()
                    .enumerate()
                    .filter_map(|(i, h)| if numeric_headers[i] { Some(h.clone()) } else { None })
                    .collect(),
            });

            total_cells += features.len(); // Увеличиваем общее количество ячеек

            // Проверяем, если общее количество ячеек превышает лимит
            if total_cells > MAX_CELLS {
                println!("Достигнуто максимальное количество ячеек ({}), дальнейшие записи будут пропущены.", MAX_CELLS);
                break; // Прерываем цикл при превышении лимита
            }
        }
    }

    // Вывод справочной информации
    println!("Справочная информация о наборе данных:");
    println!("Количество записей: {}", objects.len());
    println!("Количество числовых столбцов: {}", numeric_headers.iter().filter(|&&x| x).count());

    if objects.is_empty() {
        return Err("Нет данных для создания Dataset".into());
    }

    Dataset::new(objects).map_err(|e| e.into())
}
