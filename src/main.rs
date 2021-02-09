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
        .author("Kevin K. <kbknapp@gmail.com>")
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
        .get_matches();


    let g = graph::Graph::fromFile(matches.value_of("input").unwrap());
    if let Some(filename) = matches.value_of("output")
    {
        g.drawToFile(filename);
    }
    else
    {
        g.drawToStdout();
    }
}
