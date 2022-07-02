
pub fn get_player_color(idx: usize) -> String {

    // Array of possible player colors
    let colors = [
        "#DC143C", // crimson
        "#4169E1", // royalblue
        "#FFD700", // gold
        "#32CD32", // limegreen
        "#9370DB", // mediumpurple
        "#40E0D0", // turquoise
        "#ffb6c1", // lightpink
        "#4169E1", // royalblue
        "#ffffe0", // lightyellow
        "#90ee90", // lightgreen
        "#e0ffff", // lightcyan
        "#d3d3d3", // lightgrey
        "#8b0000", // darkred
        "#00008b", // darkblue
        "#006400", // darkgreen
        "#ff8c00", // darkorange
        "#9932cc", // darkorchid
        "#008b8b", // darkcyan
        "#a9a9a9", // darkgrey
        "#bdb76b", // darkkhaki
        "#8b008b", // darkmagenta
        "#556b2f", // darkolivegreen
        "#e9967a", // darksalmon
        "#9400d3", // darkviolet
    ];

    String::from(colors[idx])
}

