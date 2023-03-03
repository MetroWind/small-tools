#![allow(non_snake_case)]

use std::collections::HashSet;

#[derive(PartialEq, Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Point
{
    pub x: f64,
    pub y: f64,
}

impl Point
{
    pub fn new(x: f64, y: f64) -> Self
    {
        Self {x, y}
    }

    pub fn origin() -> Self
    {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(PartialEq, Clone)]
#[cfg_attr(test, derive(Debug))]
struct Circle
{
    center: Point,
    radius: f64,
}

impl Circle
{
    fn enclose(&self, p: &Point) -> bool
    {
        let dx = p.x - self.center.x;
        let dy = p.y - self.center.y;
        self.radius * self.radius > dx * dx + dy * dy
    }
}

pub struct Rect
{
    pub p0: Point,
    pub width: f64,
    pub height: f64,
}

type PointIndex = usize;
type TriangleIndex = usize;
type Edge = (PointIndex, PointIndex);
type Triangle = [PointIndex; 3];
type Polygon = Vec<Point>;

fn makeTriangle(i: usize, j: usize, k: usize) -> Triangle
{
    if i < j
    {
        if j < k
        {
            [i, j, k]
        }
        else if i < k
        {
            [i, k, j]
        }
        else
        {
            [k, i, j]
        }
    }
    else // j < i
    {
        if i < k
        {
            [j, i, k]
        }
        else if j < k
        {
            [j, k, i]
        }
        else
        {
            [i, k, j]
        }
    }
}

fn edgeInTriangle(edge: Edge, triangle: Triangle) -> bool
{
    (edge.0 == triangle[0] || edge.0 == triangle[1] || edge.0 == triangle[2]) &&
        (edge.1 == triangle[0] || edge.1 == triangle[1] || edge.1 == triangle[2])
}

fn circumCircle(p1: &Point, p2: &Point, p3: &Point) -> Option<Circle>
{
    let sq1 = p1.x * p1.x + p1.y * p1.y;
    let sq2 = p2.x * p2.x + p2.y * p2.y;
    let sq3 = p3.x * p3.x + p3.y * p3.y;
    let a = p1.x * p2.y + p3.x * p1.y + p2.x * p3.y -
        p3.x * p2.y - p2.x * p1.y - p1.x * p3.y;
    let bx = -sq1 * p2.y - sq2 * p3.y - sq3 * p1.y +
        sq3 * p2.y + sq2 * p1.y + sq1 * p3.y;
    let by = sq1 * p2.x + sq2 * p3.x + sq3 * p1.x -
        sq3 * p2.x - sq2 * p1.x - sq1 * p3.x;
    let c = -sq1 * p2.x * p3.y - p1.x * p2.y * sq3 - sq2 * p3.x * p1.y +
        sq3 * p2.x * p1.y + p3.x * p2.y * sq1 + sq2 * p1.x * p3.y;
    let p = Point::new(-bx / (2.0 * a), -by / (2.0 * a));
    let stuff = bx*bx + by*by - 4.0 * a * c;
    if stuff >= 0.0
    {
        let c = Circle { center: p, radius: stuff.sqrt() / (2.0 * a.abs()) };
        // println!(r#"<circle cx="{}" cy="{}" r="{}" stroke="grey" fill="none"/>"#,
        //          c.center.x, c.center.y, c.radius);
        Some(c)
    }
    else
    {
        None
    }
}

pub struct Triangulation
{
    points: Vec<Point>,
    triangles: Vec<Triangle>,
    edges: Vec<Edge>,
}

fn makeEdge(p1: usize, p2: usize) -> Edge
{
    if p1 < p2
    {
        (p1, p2)
    }
    else
    {
        (p2, p1)
    }
}

fn drawSVGLine(p1: &Point, p2: &Point) -> String
{
    format!(r##"<line x1="{}" x2="{}" y1="{}" y2="{}" stroke="#e0e0e0" stroke-width="1"/>"##,
            p1.x, p2.x, p1.y, p2.y)
}

fn drawPoint(p: &Point) -> String
{
    format!(r#"<circle cx="{}" cy="{}" r="2" stroke="none" fill="red"/>"#,
            p.x, p.y)
}

impl Triangulation
{
    pub fn debugSVG(&self) -> String
    {
        let xmin = self.points.iter().min_by(|p1, p2| p1.x.total_cmp(&p2.x)).unwrap().x;
        let xmax = self.points.iter().max_by(|p1, p2| p1.x.total_cmp(&p2.x)).unwrap().x;
        let ymin = self.points.iter().min_by(|p1, p2| p1.y.total_cmp(&p2.y)).unwrap().y;
        let ymax = self.points.iter().max_by(|p1, p2| p1.y.total_cmp(&p2.y)).unwrap().y;
        let mut lines = Vec::new();
        lines.push(format!(r#"<svg version="1.1" viewBox="{} {} {} {}"
xmlns="http://www.w3.org/2000/svg">"#, xmin, ymin, xmax - xmin, ymax - ymin));
        lines.push(self.debugSVGTriangles());
        for p in &self.points
        {
            lines.push(drawPoint(p));
        }
        lines.push("</svg>".to_owned());
        lines.join("\n")
    }

    pub fn debugSVGTriangles(&self) -> String
    {
        let lines: Vec<String> = self.edges.iter().map(
            |edge| drawSVGLine(&self.points[edge.0], &self.points[edge.1]))
        .collect();
        lines.join("\n")
    }

    fn trisWithPoint(&self, p: PointIndex) -> Vec<TriangleIndex>
    {
        let mut tris = Vec::new();
        for ti in 0..self.triangles.len()
        {
            if self.triangles[ti].contains(&p)
            {
                tris.push(ti);
            }
        }
        tris
    }

    fn triHasEdge(&self, tri: TriangleIndex, edge: Edge) -> bool
    {
        let t = self.triangles[tri];
        t.contains(&edge.0) && t.contains(&edge.1)
    }

    fn trisWithPointOrdered(&self, p: PointIndex) -> Vec<TriangleIndex>
    {
        let tris = self.trisWithPoint(p);
        let mut result = Vec::with_capacity(tris.len());
        result.push(tris[0]);
        let mut ti = tris[0];
        let tri = self.triangles[ti];
        let (mut edge, end_edge) = if tri[0] == p
        {
            ((p, tri[1]), (p, tri[2]))
        }
        else if tri[1] == p
        {
            ((p, tri[0]), (p, tri[2]))
        }
        else
        {
            ((p, tri[0]), (p, tri[1]))
        };

        loop
        {
            let mut end = true;
            for next_ti in &tris
            {
                if *next_ti == ti
                {
                    continue;
                }
                if self.triHasEdge(*next_ti, edge)
                {
                    // Find next edge
                    let tri = self.triangles[*next_ti];
                    let other_vert = tri.iter().find(
                        |pi| **pi != edge.0 && **pi != edge.1).unwrap();
                    edge = (p, *other_vert);

                    ti = *next_ti;
                    result.push(*next_ti);
                    end = false;
                    if self.triHasEdge(*next_ti, end_edge)
                    {
                        end = true;
                    }
                    break;
                }
            }
            if end
            {
                break;
            }
        }
        result
    }
}

/// Find all edges among a set of triangles that are not shared.
fn freeEdges(triangles: &[Triangle]) -> Vec<Edge>
{
    let mut poly: Vec<Edge> = Vec::new();
    for j in 0..triangles.len()
    {
        let mut free_edge = true;
        let edge = (triangles[j][0], triangles[j][1]);
        for k in 0..triangles.len()
        {
            if j == k
            {
                continue;
            }

            if edgeInTriangle(edge, triangles[k])
            {
                free_edge = false;
                break;
            }
        }
        if free_edge
        {
            poly.push(edge);
        }
        let mut free_edge = true;
        let edge = (triangles[j][1], triangles[j][2]);
        for k in 0..triangles.len()
        {
            if j == k
            {
                continue;
            }

            if edgeInTriangle(edge, triangles[k])
            {
                free_edge = false;
                break;
            }
        }
        if free_edge
        {
            poly.push(edge);
        }
        let mut free_edge = true;
        let edge = (triangles[j][2], triangles[j][0]);
        for k in 0..triangles.len()
        {
            if j == k
            {
                continue;
            }

            if edgeInTriangle(edge, triangles[k])
            {
                free_edge = false;
                break;
            }
        }
        if free_edge
        {
            poly.push(edge);
        }
    }
    // println!("Found {} free edges from {} triangles.", poly.len(), triangles.len());
    // println!("Free edges in {:?} are {:?}.", triangles, poly);
    poly
}

pub fn triBowyerWatson<I>(points: I, region: &Rect) -> Triangulation
    where I: Iterator<Item=Point>
{
    let mut ps: Vec<Point> = points.collect();
    let point_count = ps.len();
    let mut tris: HashSet<Triangle> = HashSet::new();
    ps.push(Point::new(region.p0.x - 10.0, region.p0.y - 10.0));
    ps.push(Point::new(region.p0.x - 10.0,
                       region.p0.y + region.height + region.height + 10.0));
    ps.push(Point::new(region.p0.x + region.width + region.width + 10.0,
                       region.p0.y - 10.0));
    tris.insert(makeTriangle(point_count, point_count+1, point_count + 2));

    for i in 0..point_count
    {
        // println!("Looking at point {} with {} triangles...", i, tris.len());
        let mut bad_tris: Vec<Triangle> = Vec::new();
        for tri in &tris
        {
            // println!("Considering triangle {:?}...", tri);
            if let Some(circ) = circumCircle(
                &ps[tri[0]], &ps[tri[1]], &ps[tri[2]])
            {
                if circ.enclose(&ps[i])
                {
                    bad_tris.push(tri.clone());
                    // println!("It's bad.");
                }
            }
        }
        let poly = freeEdges(&bad_tris);
        for tri in bad_tris
        {
            tris.remove(&tri);
        }
        // println!("After remove bad tris, there are {} triangles: {:?}", tris.len(), tris);
        for edge in poly
        {
            // println!("Refilling triangle {} -> {}, {} -> {}...", i, edge.0, i, edge.1);
            let tri = makeTriangle(i, edge.0, edge.1);
            tris.insert(tri);
        }
        // println!("After refill, there are {} triangles.", tris.len());
    }
    let final_tris: Vec<Triangle> = tris.drain().filter(
        |tri| tri[0] < point_count && tri[1] < point_count &&
            tri[2] < point_count).collect();

    let mut edges: HashSet<Edge> = HashSet::new();
    for tri in &final_tris
    {
        edges.insert(makeEdge(tri[0], tri[1]));
        edges.insert(makeEdge(tri[1], tri[2]));
        edges.insert(makeEdge(tri[2], tri[0]));
    }

    // let final_tris: Vec<Triangle> = tris.drain().collect();
    Triangulation {
        points: ps.drain(0..point_count).collect(),
        triangles: final_tris,
        edges: edges.drain().collect()
    }
}

pub struct Voronoi
{
    points: Vec<Point>,
    polygons: Vec<Polygon>,
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

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn testCircumCircle()
    {
        assert_eq!(circumCircle(&Point::new(6.0, 0.0), &Point::origin(),
                                &Point::new(0.0, 8.0)),
                   Some(Circle { center: Point::new(3.0, 4.0), radius: 5.0 }));
    }
}
