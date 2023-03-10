use crate::geometry::*;
use crate::triangulation::*;

pub struct Voronoi
{
    pub points: Vec<Point>,
    pub polygons: Vec<Polygon>,
}

impl Voronoi
{
    pub fn fromTriangulation(trian: Triangulation) -> Self
    {
        let circ_by_tri: Vec<Circle> = trian.triangles.iter().map(
            |tri| circumCircle(
                &trian.points[tri[0]], &trian.points[tri[1]], &trian.points[tri[2]])
                .unwrap()).collect();

        let mut polys = Vec::new();
        for pi in 0..trian.points.len()
        {
            let tris = trian.trisWithPointOrdered(pi);
            let mut cell = Polygon::with_capacity(tris.len());
            for ti in tris
            {
                cell.push(circ_by_tri[ti].center.clone());
            }
            polys.push(cell);
        }
        Self { points: trian.points, polygons: polys }
    }

    pub fn count(&self) -> usize
    {
        self.points.len()
    }

    pub fn debugSVGHeader(&self) -> String
    {
        let mut xmin = f64::INFINITY;
        let mut xmax = f64::NEG_INFINITY;
        let mut ymin = f64::INFINITY;
        let mut ymax = f64::NEG_INFINITY;

        for p in &self.points
        {
            if p.x < xmin
            {
                xmin = p.x;
            }
            if p.x > xmax
            {
                xmax = p.x;
            }
            if p.y < ymin
            {
                ymin = p.y;
            }
            if p.y > ymax
            {
                ymax = p.y;
            }
        }
        format!(r#"<svg version="1.1" viewBox="{} {} {} {}"
xmlns="http://www.w3.org/2000/svg">"#, xmin, ymin, xmax - xmin, ymax - ymin)
    }

    pub fn debugSVG(&self) -> String
    {
        let lines = vec![self.debugSVGHeader(), self.debugSVGPolygons(),
                         self.debugSVGPoints(), self.debugSVGFooter()];
        lines.join("\n")
    }

    pub fn debugSVGPolygons(&self) -> String
    {
        let mut lines = Vec::new();
        for poly in &self.polygons
        {
            let coord_strs: Vec<String> = poly.iter().map(
                |p| format!("{},{}", p.x, p.y)).collect();
            let points_str = coord_strs.join(" ");
            lines.push(format!(r#"<polygon points="{}" fill="none" stroke="black" />"#,
                               points_str));
        }
        lines.join("\n")
    }

    pub fn debugSVGPoints(&self) -> String
    {
        let mut lines = Vec::new();
        for p in &self.points
        {
            lines.push(drawPoint(p));
        }
        lines.join("\n")
    }

    pub fn debugSVGFooter(&self) -> String
    {
        String::from("</svg>")
    }
}
