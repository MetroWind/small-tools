#![allow(non_snake_case)]

use clap;

mod svg;
mod layout;
mod draw;
mod config;
mod geometry;
mod graph;

fn main()
{
    let matches = clap::App::new("My Super Program")
        .version("1.0")
        .author("MetroWind <chris.corsair@gmail.com>")
        .about("Does awesome things")
        .arg(clap::Arg::with_name("input")
             .value_name("FILE")
            .help("Read the layout from FILE")
            .required(true)
            .index(1))
        .arg(clap::Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Write output to FILE")
            .takes_value(true))
        .arg(clap::Arg::with_name("color-bg")
            .short("b")
            .long("color-bg")
            .value_name("COLOR")
            .help("Use COLOR as background color. Default: transparent")
            .takes_value(true))
        .arg(clap::Arg::with_name("color-fg")
            .short("f")
            .long("color-fg")
            .value_name("COLOR")
            .help("Use COLOR as foreground color. Default: black")
            .takes_value(true))
        .arg(clap::Arg::with_name("cell-width")
            .short("w")
            .long("cell-width")
            .value_name("X")
            .help("Set width of cells to X. Default: 80")
            .takes_value(true))
        .get_matches();

    let mut g = graph::Graph::fromFile(matches.value_of("input").unwrap());
    if let Some(color) = matches.value_of("color-bg")
    {
        g.config.color_bg = color.to_owned();
    }
    if let Some(color) = matches.value_of("color-fg")
    {
        g.config.color_fg = color.to_owned();
    }
    if let Some(x) = matches.value_of("cell-width")
    {
        g.config.cell_width = x.parse().unwrap();
    }
    if let Some(filename) = matches.value_of("output")
    {
        g.drawToFile(filename);
    }
    else
    {
        g.drawToStdout();
    }
}
