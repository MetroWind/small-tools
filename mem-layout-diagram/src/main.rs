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
    let matches = clap::Command::new("My Super Program")
        .version("1.0")
        .author("MetroWind <chris.corsair@gmail.com>")
        .about("Does awesome things")
        .arg(clap::Arg::new("input")
             .value_name("FILE")
            .help("Read the layout from FILE")
            .required(true)
            .index(1))
        .arg(clap::Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Write output to FILE"))
        .arg(clap::Arg::new("color-bg")
            .short('b')
            .long("color-bg")
            .value_name("COLOR")
            .help("Use COLOR as background color. Default: transparent"))
        .arg(clap::Arg::new("color-fg")
            .short('f')
            .long("color-fg")
            .value_name("COLOR")
            .help("Use COLOR as foreground color. Default: black"))
        .arg(clap::Arg::new("cell-width")
            .short('w')
            .long("cell-width")
            .value_name("X")
            .help("Set width of cells to X. Default: 80"))
        .get_matches();

    let mut g = graph::Graph::fromFile(matches.get_one::<String>("input").unwrap());
    if let Some(color) = matches.get_one::<String>("color-bg")
    {
        g.config.color_bg = color.to_owned();
    }
    if let Some(color) = matches.get_one::<String>("color-fg")
    {
        g.config.color_fg = color.to_owned();
    }
    if let Some(x) = matches.get_one::<f64>("cell-width")
    {
        g.config.cell_width = x.clone()
    }
    if let Some(filename) = matches.get_one::<String>("output")
    {
        g.drawToFile(filename);
    }
    else
    {
        g.drawToStdout();
    }
}
