static BLANK: &str = " ";
static FILLED: &str = "█";
static HALF_RIGHT: &str = "▌";
static HALF_LEFT: &str = "▐";
static ZERO: &str = "|";

pub fn compute_unicode_gauge(percent: i32) -> String {
    if percent == 0 {
        gauge(0, 0, 0, 0)
    } else {
        let filled_count: f64 = percent.abs().try_into().unwrap();
        let filled_count: usize = (filled_count / 2.5).round() as usize;
        let half_block_count = filled_count % 2;
        let filled_count: usize = filled_count / 2;
        if percent < 0 {
            gauge(half_block_count, filled_count, 0, 0)
        } else {
            gauge(0, 0, filled_count, half_block_count)
        }
    }
}

fn gauge(left_half: usize, left_filled: usize, right_filled: usize, right_half: usize) -> String {
    vec![
        vec![BLANK; 20 - left_half - left_filled],
        vec![HALF_LEFT; left_half],
        vec![FILLED; left_filled],
        vec![ZERO],
        vec![FILLED; right_filled],
        vec![HALF_RIGHT; right_half],
        vec![BLANK; 20 - right_filled - right_half],
    ]
    .concat()
    .join("")
}
