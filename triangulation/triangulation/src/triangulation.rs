use std::hash::Hash;
use std::collections::HashSet;
use std::collections::HashMap;

use crate::geometry::*;

pub fn makeEdge(p1: usize, p2: usize) -> Edge
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

pub fn drawSVGLine(p1: &Point, p2: &Point) -> String
{
    format!(r##"<line x1="{}" x2="{}" y1="{}" y2="{}" stroke="#e0e0e0" stroke-width="1"/>"##,
            p1.x, p2.x, p1.y, p2.y)
}

pub fn drawPoint(p: &Point) -> String
{
    format!(r#"<circle cx="{}" cy="{}" r="2" stroke="none" fill="red"/>"#,
            p.x, p.y)
}

pub struct Triangulation
{
    pub points: Vec<Point>,
    pub triangles: Vec<Triangle>,
    edge_sharing: HashMap<Edge, Vec<TriangleIndex>>,
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
        let lines: Vec<String> = self.edge_sharing.keys().map(
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

    fn edgesInTriWithPoint(&self, ti: TriangleIndex, p: PointIndex) ->
        (Edge, Edge)
    {
        let tri = self.triangles[ti];
        if tri[0] == p
        {
            (makeEdge(p, tri[1]), makeEdge(p, tri[2]))
        }
        else if tri[1] == p
        {
            (makeEdge(p, tri[0]), makeEdge(p, tri[2]))
        }
        else
        {
            (makeEdge(p, tri[0]), makeEdge(p, tri[1]))
        }
    }

    fn edgeIsShared(&self, edge: &Edge) -> bool
    {
        self.edge_sharing[edge].len() > 1
    }

    pub fn trisWithPointOrdered(&self, p: PointIndex) -> Vec<TriangleIndex>
    {
        let tris = self.trisWithPoint(p);
        let mut result = Vec::with_capacity(tris.len());
        result.push(tris[0]);
        let mut ti = tris[0];
        let (mut edge, mut end_edge) = self.edgesInTriWithPoint(ti, p);
        let mut open_edge = false;
        for ti in &tris
        {
            let (e1, e2) = self.edgesInTriWithPoint(*ti, p);
            if !self.edgeIsShared(&e1)
            {
                if open_edge
                {
                    end_edge = e1;
                }
                else
                {
                    edge = e1;
                    open_edge = true;
                }
            }
            else if !self.edgeIsShared(&e2)
            {
                if open_edge
                {
                    end_edge = e2;
                }
                else
                {
                    edge = e2;
                    open_edge = true;
                }
            }
        }

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

fn addToMapOfVecs<KeyType, ValueType>(
    map: &mut HashMap<KeyType, Vec<ValueType>>, key: KeyType, value: ValueType)
    where KeyType: Eq + Hash
{
    if let Some(ve) = map.get_mut(&key)
    {
        ve.push(value);
    }
    else
    {
        map.insert(key, vec![value]);
    }
}

pub fn triBowyerWatson<I>(points: I) -> Triangulation
    where I: Iterator<Item=Point>
{
    let mut ps: Vec<Point> = points.collect();
    let region = Rect::bboxOfPoints(&ps);
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

    let mut edge_sharing: HashMap<Edge, Vec<TriangleIndex>> = HashMap::new();
    for ti in 0..final_tris.len()
    {
        let tri = final_tris[ti];
        addToMapOfVecs(&mut edge_sharing, makeEdge(tri[0], tri[1]), ti);
        addToMapOfVecs(&mut edge_sharing, makeEdge(tri[0], tri[2]), ti);
        addToMapOfVecs(&mut edge_sharing, makeEdge(tri[1], tri[2]), ti);
    }

    // let final_tris: Vec<Triangle> = tris.drain().collect();
    Triangulation {
        points: ps.drain(0..point_count).collect(),
        triangles: final_tris,
        edge_sharing,
    }
}
