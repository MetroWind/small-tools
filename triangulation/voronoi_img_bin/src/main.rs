use std::fs::File;
use std::io::prelude::*;

use anyhow::Result;
use image::io::Reader as ImageReader;
use voronoi_img::VoronoiImage;

fn main() -> Result<()>
{
    let args = clap::Command::new("Generate voronoi color cells from an image")
        .author("MetroWind")
        .arg(clap::Arg::new("image").value_name("FILE").required(true)
             .help("The image to generate voronoi cells from"))
        .arg(clap::Arg::new("count")
             .short('c').long("count").value_name("N")
             .value_parser(clap::value_parser!(u32))
             .default_value("500")
             .help("The number of cells to generate."))
        .arg(clap::Arg::new("output")
             .short('o').long("output").value_name("FILE")
             .help("The path of the output file. Default or \"-\" is to print to stdout."))
        .get_matches();
    let img = ImageReader::open(&args.get_one::<String>("image").unwrap())?
        .decode()?;
    let vor = VoronoiImage::fromImage(
        &img, *args.get_one::<u32>("count").unwrap());
    let svg = vor.svg();
    if let Some(output) = args.get_one::<String>("output")
    {
        if output == "-"
        {
            println!("{}", svg);
        }
        else
        {
            File::create(output)?.write_all(svg.as_bytes())?;
        }
    }
    else
    {
        println!("{}", svg);
    }

    Ok(())
}
