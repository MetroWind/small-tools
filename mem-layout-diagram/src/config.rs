#[derive(Clone)]
pub struct Config
{
    pub cell_width: f64,
    pub cell_height: f64,
    pub canvas_padding: f64,
    pub font_size: f64,
    pub arrow_size: f64,
    pub byte_sep_size: f64,
    pub color_fg: String,
    pub color_bg: String,
    pub font_family: String,
}

impl Default for Config
{
    fn default() -> Self
    {
        Self {
            cell_width: 80.0,
            cell_height: 20.0,
            canvas_padding: 2.0,
            font_size: 14.0,
            arrow_size: 8.0,
            byte_sep_size: 4.0,
            color_fg: "black".to_owned(),
            color_bg: "white".to_owned(),
            font_family: "'Rec Mono Duotone', 'IBM Plex Mono', Iosevka, Inconsolata, monospace".to_owned(),
        }
    }
}
