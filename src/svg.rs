use crate::geometry;
use crate::config;

pub enum TextAlign
{
    Left, Middle, Right
}

pub struct SVGDrawer<'a>
{
    config: &'a config::Config,
}

impl<'a> SVGDrawer<'a>
{
    pub fn new(conf: &'a config::Config) -> Self
    {
        Self{ config: conf }
    }

    pub fn header(&self, bbox: &geometry::Region) -> String
    {
        let x0 = bbox.xmin - self.config.canvas_padding;
        let y0 = bbox.ymin - self.config.canvas_padding;
        let width = bbox.xmax - bbox.xmin + 2.0 * self.config.canvas_padding;
        let height = bbox.ymax - bbox.ymin + 2.0 * self.config.canvas_padding;
        format!(r#"<svg viewBox="{} {} {} {}" width="{}" height="{}"
xmlns="http://www.w3.org/2000/svg">
<rect x="{}" y="{}" width="100%" height="100%" fill="{}" />
"#,
                x0, y0, width, height, width, height, x0, y0,
                self.config.color_bg)
    }

    pub fn footer(&self) -> String
    {
        String::from("</svg>")
    }

    pub fn line(&self, begin: (f64, f64), end: (f64, f64)) -> String
    {
        format!(r#"<line x1="{}" y1="{}" x2="{}" y2="{}"
stroke="{}" stroke-width="2" />"#, begin.0, begin.1, end.0, end.1,
                self.config.color_fg)
    }

    pub fn dashedLine(&self, begin: (f64, f64), end: (f64, f64)) -> String
    {
        format!(r#"<line x1="{}" y1="{}" x2="{}" y2="{}"
stroke="{}" stroke-width="2" stroke-dasharray="5, 5" />"#,
                begin.0, begin.1, end.0, end.1, self.config.color_fg)
    }

    pub fn text(&self, content: &str, anchor: (f64, f64), align: TextAlign) -> String
    {
        format!(r#"<text x="{}" y="{}" text-anchor="{}" font-size="{}"
font-family="{}" fill="{}">{}</text>"#,
                anchor.0, anchor.1,
                match align
                {
                    TextAlign::Left => "start",
                    TextAlign::Middle => "middle",
                    TextAlign::Right => "end",
                },
                self.config.font_size,
                self.config.font_family,
                self.config.color_fg,
                content)
    }

    pub fn curveCubic(&self, begin: (f64, f64), c1: (f64, f64), c2: (f64, f64),
                      end: (f64, f64)) -> String
    {
        format!(r#"<path d="M {},{} C {},{} {},{} {},{}" stroke="{}"
stroke-width="2" fill="none" />"#,
                begin.0, begin.1, c1.0, c1.1, c2.0, c2.1, end.0, end.1,
                self.config.color_fg)
    }

    // Draw an arrow head whose tip is at `pos` and pointing to `dir`
    // direction. Argument `dir` is a angle in degree whose zero is at
    // +x, and rotates from +x to +y.
    pub fn arrowHead(&self, pos: (f64, f64), dir: f64) -> String
    {
        let angle1 = (dir + 150.0).to_radians();
        let angle2 = (dir - 150.0).to_radians();
        let line1_end = (pos.0 + self.config.arrow_size * angle1.cos(),
                         pos.1 + self.config.arrow_size * angle1.sin());
        let line2_end = (pos.0 + self.config.arrow_size * angle2.cos(),
                         pos.1 + self.config.arrow_size * angle2.sin());
        format!(r#"<line x1="{pos_x}" y1="{pos_y}" x2="{end1_x}" y2="{end1_y}"
stroke="{fg}" stroke-width="2" />
<line x1="{pos_x}" y1="{pos_y}" x2="{end2_x}" y2="{end2_y}"
stroke="{fg}" stroke-width="2" />
"#,
                pos_x=pos.0, pos_y=pos.1,
                end1_x=line1_end.0, end1_y=line1_end.1,
                end2_x=line2_end.0, end2_y=line2_end.1,
                fg=self.config.color_fg)
    }
}
