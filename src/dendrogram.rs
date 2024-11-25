use crate::data::ClusterMerge;
use plotters::prelude::*;
use std::fs::File;

pub fn plot_dendrogram(trace: &[ClusterMerge]) {
    println!("Построение дендрограммы:");
    for step in trace {
        println!(
            "Объединение кластеров {} и {} на расстоянии {}",
            step.cluster1, step.cluster2, step.distance
        );
    }
}


pub fn plot_dendrogram_to_png(trace: &[ClusterMerge], output_path: &str) {
    use plotters::prelude::*;

    let root = BitMapBackend::new(output_path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Dendrogram", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..(trace.len() * 2), 0.0..trace.last().unwrap().distance * 1.2)
        .unwrap();

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("Cluster")
        .y_desc("Distance")
        .draw()
        .unwrap();

    let mut cluster_positions = vec![0.0; trace.len() * 2];
    let mut heights = vec![0.0; trace.len() * 2];

    // Инициализация позиций для начальных кластеров
    for i in 0..trace.len() {
        cluster_positions[i] = i as f64;
    }

    // Рисуем дендрограмму
    for (i, merge) in trace.iter().enumerate() {
        let cluster1_pos = cluster_positions[merge.cluster1];
        let cluster2_pos = cluster_positions[merge.cluster2];

        let new_pos = (cluster1_pos + cluster2_pos) / 2.0; // Новая позиция
        cluster_positions[trace.len() + i] = new_pos;

        let height = merge.distance;
        heights[trace.len() + i] = height;

        // Рисуем вертикальные линии от каждого кластера к точке слияния
        chart
            .draw_series(LineSeries::new(
                vec![
                    (cluster1_pos as usize, heights[merge.cluster1]),
                    (cluster1_pos as usize, height),
                ]
                .into_iter(),
                &BLACK,
            ))
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                vec![
                    (cluster2_pos as usize, heights[merge.cluster2]),
                    (cluster2_pos as usize, height),
                ]
                .into_iter(),
                &BLACK,
            ))
            .unwrap();

        // Рисуем горизонтальную линию, соединяющую два кластера на уровне height
        chart
            .draw_series(LineSeries::new(
                vec![
                    (cluster1_pos as usize, height),
                    (cluster2_pos as usize, height),
                ]
                .into_iter(),
                &BLACK,
            ))
            .unwrap();
    }

    root.present().unwrap();
    println!("Dendrogram saved to {}", output_path);
}



pub fn plot_dendrogram2(merge_steps: Vec<ClusterMerge>, file_path: &str) {
    let root_area = BitMapBackend::new(file_path, (1800, 1000))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Dendrogram", ("sans-serif", 30).into_font())
        .build_cartesian_2d(0..merge_steps.len(), 0.0..1.5)
        .unwrap();

    chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc("Merge Step")
        .y_desc("Distance")
        .draw()
        .unwrap();

    // Рисуем линии для каждого шага объединения
    for (i, step) in merge_steps.iter().enumerate() {
        let x1 = step.cluster1;
        let x2 = step.cluster2;
        let y = step.distance;

        chart.draw_series(LineSeries::new(
            vec![(x1, y), (x2, y)],
            &RED,
        ))
        .unwrap();

        // Добавление горизонтальных линий
        chart.draw_series(LineSeries::new(
            vec![(i, y), (i, 0.0)],
            &BLACK,
        ))
        .unwrap();
    }
}


