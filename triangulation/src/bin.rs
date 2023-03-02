#![allow(non_snake_case)]

use rand;
use triangulation as tri;

fn randomPointInRegion(r: &tri::Rect) -> tri::Point
{
    let x: f64 = rand::random::<f64>() * r.width + r.p0.x;
    let y: f64 = rand::random::<f64>() * r.height + r.p0.y;
    tri::Point::new(x, y)
}

fn main()
{
    let region = tri::Rect {
        p0: tri::Point::new(0.0, 0.0),
        width: 500.0,
        height: 500.0
    };
    let mut ps: Vec<tri::Point> = Vec::new();
    for _ in 0..200
    {
        ps.push(randomPointInRegion(&region));
    }

    let trii = tri::triBowyerWatson(ps.drain(..), &region);
    println!("{}", trii.debugSVG());
}
