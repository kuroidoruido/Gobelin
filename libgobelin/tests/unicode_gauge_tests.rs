use libgobelin::compute_unicode_gauge;
use std::collections::BTreeMap;

#[test]
fn it_should_display_correctly_percent_as_gauge() {
    let tests: BTreeMap<i32, &str> = BTreeMap::from([
        (-100, "████████████████████|                    "),
        (-95, " ███████████████████|                    "),
        (-90, "  ██████████████████|                    "),
        (-85, "   █████████████████|                    "),
        (-80, "    ████████████████|                    "),
        (-75, "     ███████████████|                    "),
        (-70, "      ██████████████|                    "),
        (-65, "       █████████████|                    "),
        (-60, "        ████████████|                    "),
        (-55, "         ███████████|                    "),
        (-50, "          ██████████|                    "),
        (-45, "           █████████|                    "),
        (-40, "            ████████|                    "),
        (-35, "             ███████|                    "),
        (-30, "              ██████|                    "),
        (-25, "               █████|                    "),
        (-20, "                ████|                    "),
        (-15, "                 ███|                    "),
        (-10, "                  ██|                    "),
        (-5, "                   █|                    "),
        (0, "                    |                    "),
        (5, "                    |█                   "),
        (10, "                    |██                  "),
        (15, "                    |███                 "),
        (20, "                    |████                "),
        (25, "                    |█████               "),
        (30, "                    |██████              "),
        (35, "                    |███████             "),
        (40, "                    |████████            "),
        (45, "                    |█████████           "),
        (50, "                    |██████████          "),
        (55, "                    |███████████         "),
        (60, "                    |████████████        "),
        (65, "                    |█████████████       "),
        (70, "                    |██████████████      "),
        (75, "                    |███████████████     "),
        (80, "                    |████████████████    "),
        (85, "                    |█████████████████   "),
        (90, "                    |██████████████████  "),
        (95, "                    |███████████████████ "),
        (100, "                    |████████████████████"),
        (99, "                    |███████████████████ "),
        (98, "                    |███████████████████ "),
        (97, "                    |███████████████████ "),
        (96, "                    |███████████████████ "),
        (94, "                    |██████████████████  "),
        (93, "                    |██████████████████  "),
        (92, "                    |██████████████████  "),
        (91, "                    |██████████████████  "),
    ]);
    for (percent, gauge) in tests.iter() {
        let actual = compute_unicode_gauge(percent.clone());
        assert!(
            actual == String::from(gauge.clone()),
            "{} should be displayed with gauge {} not {}",
            percent,
            gauge.clone(),
            actual
        );
    }
}
