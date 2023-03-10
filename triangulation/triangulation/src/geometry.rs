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
pub struct Circle
{
    pub center: Point,
    pub radius: f64,
}

impl Circle
{
    pub fn enclose(&self, p: &Point) -> bool
    {
        let dx = p.x - self.center.x;
        let dy = p.y - self.center.y;
        self.radius * self.radius > dx * dx + dy * dy
    }
}

fn wrapPeriodic(x0: f64, size: f64, x: f64) -> f64
{
    x - ((x - x0) / size).floor() * size
}

pub struct Rect
{
    pub p0: Point,
    pub width: f64,
    pub height: f64,
}

impl Rect
{
    pub fn new(p0: Point, width: f64, height: f64) -> Self
    {
        Self {p0, width, height}
    }

    pub fn bboxOfPoints(ps: &[Point]) -> Self
    {
        let xmin = ps.iter().min_by(|p1, p2| p1.x.total_cmp(&p2.x)).unwrap().x;
        let xmax = ps.iter().max_by(|p1, p2| p1.x.total_cmp(&p2.x)).unwrap().x;
        let ymin = ps.iter().min_by(|p1, p2| p1.y.total_cmp(&p2.y)).unwrap().y;
        let ymax = ps.iter().max_by(|p1, p2| p1.y.total_cmp(&p2.y)).unwrap().y;
        Self::new(Point::new(xmin, ymin), xmax - xmin, ymax - ymin)
    }

    pub fn wrapPointPeriodic(&self, p: &Point) -> Point
    {
        Point::new(wrapPeriodic(self.p0.x, self.width, p.x),
                   wrapPeriodic(self.p0.y, self.height, p.y))
    }
}

pub type PointIndex = usize;
pub type TriangleIndex = usize;
pub type Edge = (PointIndex, PointIndex);
pub type Triangle = [PointIndex; 3];
pub type Polygon = Vec<Point>;

pub fn makeTriangle(i: usize, j: usize, k: usize) -> Triangle
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

pub fn edgeInTriangle(edge: Edge, triangle: Triangle) -> bool
{
    (edge.0 == triangle[0] || edge.0 == triangle[1] || edge.0 == triangle[2]) &&
        (edge.1 == triangle[0] || edge.1 == triangle[1] || edge.1 == triangle[2])
}

pub fn circumCircle(p1: &Point, p2: &Point, p3: &Point) -> Option<Circle>
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

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn testPeriodicWrap()
    {
        let rect = Rect::new(Point::new(1.0, 1.0), 2.0, 3.0);
        assert_eq!(rect.wrapPointPeriodic(&Point::new(1.5, 1.5)),
                   Point::new(1.5, 1.5));
        assert_eq!(rect.wrapPointPeriodic(&Point::origin()),
                   Point::new(2.0, 3.0));
        assert_eq!(rect.wrapPointPeriodic(&Point::new(10.0, 100.0)),
                   Point::new(2.0, 1.0));
    }
}
