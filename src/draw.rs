use crate::layout;
use crate::svg;
use crate::config;
use crate::geometry;

fn scopeBeginY(column: &layout::Column, scope_name: &str, conf: &config::Config)
    -> Option<f64>
{
    let mut scope_begin_y: f64 = 0.0;

    for scope in &column.scopes
    {
        if scope.name == scope_name
        {
            return Some(scope_begin_y);
        }
        scope_begin_y += scope.size() as f64 * conf.cell_height;
        scope_begin_y += 3.0 * conf.cell_height;
    }
    None
}

fn cellBeginY(column: &layout::Column, scope: &str, cell: &str,
              conf: &config::Config) -> Option<f64>
{
    let index_maybe = column.findCellIndex(scope, cell);
    if index_maybe.is_none()
    {
        return None;
    }

    let index = index_maybe.unwrap();
    let scope_begin_y = scopeBeginY(column, scope, conf).unwrap();

    Some(scope_begin_y
         + column.scopes[index.scope].cellBegin(index.cell) as f64
         * conf.cell_height)
}

pub struct Canvas<'a>
{
    result: Vec<String>,
    config: &'a config::Config,
    bbox: geometry::Region,
    drawer: svg::SVGDrawer<'a>,
}

impl<'a> Canvas<'a>
{
    pub fn newWithConfig(conf: &'a config::Config) -> Self
    {
        Self{ result: Vec::new(), config: conf,
              bbox: geometry::Region::new(),
              drawer: svg::SVGDrawer::new(&conf),
        }
    }

    pub fn drawScope(&mut self, scope: &layout::Scope, anchor: (f64, f64))
    {
        // The two vertical lines
        self.result.push(self.drawer.line(
            (anchor.0, -self.config.cell_height * 0.5 + anchor.1),
            (anchor.0, self.config.cell_height * 0.5 +
             scope.size() as f64 * self.config.cell_height + anchor.1)));
        self.result.push(self.drawer.line(
            (anchor.0 + self.config.cell_width,
             -self.config.cell_height * 0.5 + anchor.1),
            (anchor.0 + self.config.cell_width, self.config.cell_height * 0.5 +
             scope.size() as f64 * self.config.cell_height + anchor.1)));

        // The first (top-most) horizontal line.
        self.result.push(self.drawer.line(
            (anchor.0, anchor.1), (anchor.0 + self.config.cell_width, anchor.1)));

        {
            let mut y = anchor.1;

            for cell in &scope.cells
            {
                // If a cell size > 1, draw two small lines at each
                // byte boundary.
                for i in 1..cell.size
                {
                    self.result.push(self.drawer.line(
                        (anchor.0, y + self.config.cell_height * i as f64),
                        (anchor.0 + self.config.byte_sep_size,
                         y + self.config.cell_height * i as f64)));
                    self.result.push(self.drawer.line(
                        (anchor.0 + self.config.cell_width
                         - self.config.byte_sep_size,
                         y + self.config.cell_height * i as f64),
                        (anchor.0 + self.config.cell_width,
                         y + self.config.cell_height * i as f64)));
                }

                let text_y = y + self.config.cell_height
                    - self.config.cell_height * 0.25;
                // Draw label
                if !cell.label.is_empty()
                {
                    self.result.push(self.drawer.text(
                        &cell.label,
                        (anchor.0 - self.config.font_size * 0.5, text_y),
                        svg::TextAlign::Right));
                }
                // Draw cell content
                if !cell.content.is_empty()
                {
                    self.result.push(self.drawer.text(
                        &cell.content,
                        (anchor.0 + self.config.font_size * 0.2, text_y),
                        svg::TextAlign::Left));
                }
                // Draw cell address
                if cell.showAddress()
                {
                    self.result.push(self.drawer.text(
                        &cell.address,
                        (anchor.0 + self.config.cell_width
                         + self.config.font_size * 0.2, text_y),
                        svg::TextAlign::Left));
                }

                y += cell.size as f64 * self.config.cell_height;

                // For each cell, draw its bottom horizontal line
                self.result.push(self.drawer.line(
                    (anchor.0, y), (anchor.0 + self.config.cell_width, y)));
            }
        }

        self.bbox.enlarge(&geometry::Region{
            xmin: anchor.0 - self.config.cell_width,
            xmax: anchor.0 + self.config.cell_width,
            ymin: -self.config.cell_height * 0.5 + anchor.1,
            ymax: self.config.cell_height * 0.5 +
                scope.size() as f64 * self.config.cell_height + anchor.1,
        });
    }

    pub fn drawColumn(&mut self, column: &layout::Column, anchor: (f64, f64))
    {
        self.drawScope(&column.scopes[0], anchor);
        for i in 1..column.scopes.len()
        {
            let y_end = anchor.1 + (column.scopes[i-1].size() as f64 + 0.5)
                * self.config.cell_height;
            let next_y = y_end + 2.5 * self.config.cell_height;
            self.result.push(self.drawer.dashedLine(
                (anchor.0, y_end),
                (anchor.0, next_y - 0.5 * self.config.cell_height)));
            self.result.push(self.drawer.dashedLine(
                (anchor.0 + self.config.cell_width, y_end),
                (anchor.0 + self.config.cell_width,
                 next_y - 0.5 * self.config.cell_height)));
            self.drawScope(&column.scopes[i], (anchor.0, next_y));
        }

        for pointer in &column.pointers
        {
            self.drawPointer(column, pointer, anchor);
        }

        let mut has_scope_label = false;
        // Draw scope labels
        for scope in &column.scopes
        {
            if !scope.showName()
            {
                continue;
            }

            has_scope_label = true;
            let scope_begin_y = scopeBeginY(column, &scope.name, self.config)
                .unwrap();
            let scope_end_y = scope_begin_y + scope.size() as f64
                * self.config.cell_height;
            let scope_center_y = (scope_begin_y + scope_end_y) * 0.5;
            let x = self.bbox.xmax;

            self.result.push(self.drawer.curveCubic(
                (x + 10.0, scope_begin_y),
                (x + 20.0, scope_begin_y),
                (x + 10.0, scope_center_y),
                (x + 20.0, scope_center_y)));
            self.result.push(self.drawer.curveCubic(
                (x + 10.0, scope_end_y),
                (x + 20.0, scope_end_y),
                (x + 10.0, scope_center_y),
                (x + 20.0, scope_center_y)));

            self.result.push(self.drawer.text(
                &scope.name,
                (x + 20.0 + 0.5 * self.config.font_size,
                 scope_center_y + 0.25 * self.config.font_size),
                svg::TextAlign::Left));
        }

        if has_scope_label
        {
            self.bbox.extendPositiveX(20.0 + self.config.cell_width);
        }
    }

    pub fn drawPointer(&mut self, column: &layout::Column,
                       pointer: &layout::Pointer, column_anchor: (f64, f64))
    {
        let begin_y = column_anchor.1 + cellBeginY(
            column, &pointer.from.scope, &pointer.from.cell, self.config).unwrap()
            + 0.5 * self.config.cell_height;
        let end_y = column_anchor.1 + cellBeginY(
            column, &pointer.to.scope, &pointer.to.cell, self.config).unwrap()
            + 0.5 * self.config.cell_height;

        let begin_x = if column.findCell(&pointer.from.scope, &pointer.from.cell)
            .unwrap().showAddress()
        {
            column_anchor.0 + column_anchor.0 + 2.0 * self.config.cell_width
        }
        else
        {
            column_anchor.0 + self.config.cell_width + 4.0
        };

        let end_x = if column.findCell(&pointer.to.scope, &pointer.to.cell)
            .unwrap().showAddress()
        {
            column_anchor.0 + column_anchor.0 + 2.0 * self.config.cell_width
        }
        else
        {
            column_anchor.0 + self.config.cell_width + 4.0
        };

        let curve_handle_x = begin_x.max(end_x) + self.config.cell_width;

        self.result.push(self.drawer.curveCubic(
            (begin_x, begin_y),
            (curve_handle_x, begin_y),
            (curve_handle_x, end_y),
            (end_x, end_y)));
        self.result.push(self.drawer.arrowHead((end_x, end_y), 180.0));
        self.bbox.enlarge(&geometry::Region{
            xmin: column_anchor.0,
            xmax: curve_handle_x,
            ymin: column_anchor.1,
            ymax: column_anchor.1,
        });
    }

    pub fn print(&self)
    {
        println!("{}", self.drawer.header(&self.bbox));
        for line in &self.result
        {
            println!("{}", line);
        }
        println!("{}", self.drawer.footer());
    }

    pub fn as_string(&self) -> String
    {
        let mut lines = Vec::new();
        lines.push(self.drawer.header(&self.bbox));
        lines.push(self.result.join("\n"));
        lines.push(self.drawer.footer());
        lines.join("\n")
    }
}
