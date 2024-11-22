#[derive(Debug, Clone)]
pub struct Object {
    pub id: usize,
    pub features: Vec<f64>,
    pub feature_names: Vec<String>, // Имена признаков
}

// Коллекция данных с проверкой на согласованность
#[derive(Debug)]
pub struct Dataset {
    pub objects: Vec<Object>,
    pub feature_names: Vec<String>, // Общие имена признаков для всего набора
}
impl Dataset {
    // Создание набора данных с проверкой
    pub fn new(objects: Vec<Object>) -> Result<Self, String> {
        if objects.is_empty() {
            return Err("Набор данных пуст".to_string());
        }

        let feature_count = objects[0].features.len();
        let feature_names = objects[0].feature_names.clone();

        for obj in &objects {
            if obj.features.len() != feature_count {
                return Err(format!(
                    "Объект с ID {} имеет некорректное число признаков",
                    obj.id
                ));
            }
            if obj.feature_names != feature_names {
                return Err(format!(
                    "Объект с ID {} имеет несовпадающие заголовки признаков",
                    obj.id
                ));
            }
        }

        Ok(Self {
            objects,
            feature_names,
        })
    }


    // Нормализация данных
    pub fn normalize(&self) -> Dataset {
        let mut normalized_objects = vec![];
        let num_features = self.objects[0].features.len();
        let mut min_values = vec![f64::MAX; num_features];
        let mut max_values = vec![f64::MIN; num_features];

        for obj in &self.objects {
            for (i, &val) in obj.features.iter().enumerate() {
                if val < min_values[i] {
                    min_values[i] = val;
                }
                if val > max_values[i] {
                    max_values[i] = val;
                }
            }
        }

        for obj in &self.objects {
            let normalized_features = obj
                .features
                .iter()
                .enumerate()
                .map(|(i, &val)| (val - min_values[i]) / (max_values[i] - min_values[i]))
                .collect();

            normalized_objects.push(Object {
                id: obj.id,
                features: normalized_features,
                feature_names: obj.feature_names.clone(),
            });
        }

        Dataset {
            objects: normalized_objects,
            feature_names: self.feature_names.clone(),
        }
    }
}
