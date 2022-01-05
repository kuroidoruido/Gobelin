pub fn compute_unicode_gauge(percent: i32) -> String {
    let fill_cell_count: usize = percent.abs().try_into().unwrap();
    let fill_cell_count: usize = fill_cell_count / 5;
    if percent == 0 {
        vec![vec![" "; 20], vec!["|"], vec![" "; 20]]
            .concat()
            .join("")
    } else if percent < 0 {
        vec![
            vec![" "; 20 - fill_cell_count],
            vec!["█"; fill_cell_count],
            vec!["|"],
            vec![" "; 20],
        ]
        .concat()
        .join("")
    } else {
        vec![
            vec![" "; 20],
            vec!["|"],
            vec!["█"; fill_cell_count],
            vec![" "; 20 - fill_cell_count],
        ]
        .concat()
        .join("")
    }
}
