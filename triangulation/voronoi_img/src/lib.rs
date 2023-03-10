#![allow(non_snake_case)]

use image::{DynamicImage, GenericImageView, Pixel};
use triangulation::voronoi::Voronoi;
use triangulation::geometry::{Rect, Point};
use triangulation::triangulation::triBowyerWatson;

type Rgb8 = image::Rgb<u8>;

fn genHelperPoints(bbox: &Rect, dist: f64) -> Vec<Point>
{
    let mut x = 0.0;
    let mut result = Vec::new();
    while x <= bbox.width
    {
        result.push(Point::new(x, -dist));
        result.push(Point::new(x, -2.0 * dist));
        result.push(Point::new(x, bbox.height + dist));
        result.push(Point::new(x, bbox.height + 2.0 * dist));
        x += dist;
    }
    let mut y = 0.0;
    while y <= bbox.height
    {
        result.push(Point::new(-dist, y));
        result.push(Point::new(-2.0 * dist, y));
        result.push(Point::new(bbox.width + dist, y));
        result.push(Point::new(bbox.width + 2.0 * dist, y));
        y += dist;
    }
    result
}

fn sampleColor(img: &DynamicImage, bbox: &Rect, p: &Point, half_size: u32) ->
    Rgb8
{
    let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
    let xi = p.x.floor() as i32;
    let yi = p.y.floor() as i32;
    let mut count: u32 = 0;
    let half_size = half_size as i32;
    for x in (xi - half_size)..(xi + half_size)
    {
        let x = x as f64;
        for y in (yi - half_size)..(yi + half_size)
        {
            let wrapped = bbox.wrapPointPeriodic(&Point::new(x, y as f64));
            let c = img.get_pixel(wrapped.x as u32, wrapped.y as u32).to_rgb();
            r += c[0] as u32;
            g += c[1] as u32;
            b += c[2] as u32;
            count += 1;
        }
    }
    Rgb8::from([(r / count) as u8, (g / count) as u8, (b / count) as u8,])
}

fn randomPointInRegion(r: &Rect) -> Point
{
    let x: f64 = rand::random::<f64>() * r.width + r.p0.x;
    let y: f64 = rand::random::<f64>() * r.height + r.p0.y;
    Point::new(x, y)
}

fn colorInHex(color: Rgb8) -> String
{
    format!("#{:02x}{:02x}{:02x}", color[0], color[1], color[2])
}

pub struct VoronoiImage
{
    v: Voronoi,
    c: Vec<Rgb8>,
    bbox: Rect,
}

impl VoronoiImage
{
    pub fn fromImage(img: &DynamicImage, point_count: u32) -> Self

    {
        let bbox = Rect::new(Point::origin(), img.width() as f64,
                             img.height() as f64);
        let mut helper_points = genHelperPoints(&bbox, 100.0);
        let points = (0..point_count).map(|_| randomPointInRegion(&bbox))
            .chain(helper_points.drain(..));
        let vor = Voronoi::fromTriangulation(triBowyerWatson(points));

        let colors: Vec<Rgb8> = vor.points.iter().map(
            |p| sampleColor(img, &bbox, p, 4)).collect();
        Self { v: vor, c: colors, bbox }
    }

    pub fn svg(&self) -> String
    {
        let mut lines = Vec::new();
        lines.push(format!(r#"<svg version="1.1" viewBox="{} {} {} {}"
xmlns="http://www.w3.org/2000/svg">"#, self.bbox.p0.x, self.bbox.p0.y,
                           self.bbox.width, self.bbox.height));
        lines.extend((0..self.v.count()).map(|i| {
            let poly = &self.v.polygons[i];
            let coord_strs: Vec<String> = poly.iter().map(
                |p| format!("{},{}", p.x, p.y)).collect();
            let points_str = coord_strs.join(" ");
            let color = colorInHex(self.c[i]);
            format!(r#"<polygon points="{}" fill="{}" stroke="{}" />"#,
                    points_str, color, color)
        }));
        lines.push("</svg>".to_owned());
        lines.join("\n")
    }
}
