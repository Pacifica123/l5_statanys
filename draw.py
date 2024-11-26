import matplotlib.pyplot as plt
import numpy as np
from scipy.cluster.hierarchy import dendrogram, linkage

# Функция для чтения данных из файла
def read_clusters(file_path):
    steps = []
    distances = []
    
    with open(file_path, 'r', encoding='utf-8') as file:
        for line in file:
            parts = line.split('| Кластеры: ')
            if len(parts) == 2:
                step_info = parts[0].strip()
                clusters_info = parts[1].strip()
                
                # Извлечение минимального расстояния из информации о шаге
                distance_part = step_info.split('=')[1].strip()
                distances.append(float(distance_part))
                
                steps.append((step_info, clusters_info))
    
    return steps, distances

# Функция для построения дендрограммы и сохранения в PNG
def plot_dendrogram(distances, labels, filename='dendrogram.png'):
    # Создаем матрицу расстояний для дендрограммы
    Z = linkage(np.array(distances).reshape(-1, 1), method='ward')

    plt.figure(figsize=(15, 7))
    dendrogram(Z, labels=labels)  # Используем правильные метки объектов
    plt.title('Дендрограмма')
    
    # Сохранение дендрограммы в файл PNG
    plt.savefig(filename, dpi=300)
    plt.close()

# Основной код
if __name__ == "__main__":
    file_path = 'clusters.txt'
    steps, distances = read_clusters(file_path)
    
    # Извлечение меток для дендрограммы из последнего шага
    final_labels = steps[-1][1]
    
    # Форматирование меток для отображения на графике
    labels_list = []
    for cluster in final_labels.split(' | '):
        ids_part = cluster.split(': ')[1]  # Получаем часть с ID
        ids_list = eval(ids_part)           # Преобразуем строку в список (можно использовать json.loads)
        labels_list.extend(ids_list)         # Добавляем все ID в общий список меток

    print("Идентификаторы для дендрограммы:", labels_list)  # Выводим идентификаторы перед построением

    # Строим дендрограмму и сохраняем в файл
    plot_dendrogram(distances, labels=labels_list[:len(distances)], filename='dendrogram.png')  