#![allow(non_snake_case)]

use std::collections::HashSet;

#[derive(PartialEq)]
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

#[derive(PartialEq)]
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

type Edge = (usize, usize);
type Triangle = [usize; 3];

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
    format!(r#"<line x1="{}" x2="{}" y1="{}" y2="{}" stroke="black" stroke-width="1"/>"#,
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
        let mut edges: HashSet<Edge> = HashSet::new();
        for tri in &self.triangles
        {
            let edge = makeEdge(tri[0], tri[1]);
            if !edges.contains(&edge)
            {
                lines.push(drawSVGLine(&self.points[edge.0],
                                       &self.points[edge.1]));
                edges.insert(edge);
            }
            let edge = makeEdge(tri[1], tri[2]);
            if !edges.contains(&edge)
            {
                lines.push(drawSVGLine(&self.points[edge.0],
                                       &self.points[edge.1]));
                edges.insert(edge);
            }
            let edge = makeEdge(tri[2], tri[0]);
            if !edges.contains(&edge)
            {
                lines.push(drawSVGLine(&self.points[edge.0],
                                       &self.points[edge.1]));
                edges.insert(edge);
            }
        }
        for p in &self.points
        {
            lines.push(drawPoint(p));
        }
        lines.push("</svg>".to_owned());
        lines.join("\n")
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
    // let final_tris: Vec<Triangle> = tris.drain().collect();
    Triangulation {
        points: ps.drain(0..point_count).collect(),
        triangles: final_tris,
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
