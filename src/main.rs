#![allow(non_snake_case)]
use std::io::prelude::*;
use std::collections::HashMap;

use clap;

#[macro_use]
mod error;
mod order_record;
mod config;

use error::Error;
use order_record::Order;

fn pressAnyKeyToContinue()
{
    println!("Press any key to continue...");
    std::io::stdin().read(&mut [0u8]).unwrap();
}

fn main() -> Result<(), Error>
{
    let opts = clap::App::new("Amazon order history to beancount inspector")
        .version("0.0.1")
        .author("MetroWind <chris.corsair@gmail.com>")
        .arg(clap::Arg::new("after")
            .short('a')
            .long("after")
            .value_name("ORDER")
            .about("Only find orders placed after ORDER.")
            .takes_value(true))
        .arg(clap::Arg::new("input")
             .about("The order record CSV file. You can acquire this file from https://www.amazon.com/gp/b2b/reports.")
             .required(true)
             .value_name("FILE")
            .index(1))
        .get_matches();

    let conf = config::Config::default();
    let orders = Order::fromCSV(opts.value_of("input").unwrap())?;
    let mut start_from: usize = 0;
    if let Some(after) = opts.value_of("after")
    {
        for i in 0..orders.len()
        {
            if orders[i].order_number == after
            {
                start_from = i + 1;
                break;
            }
        }
    }

    // Multiple items in the same order are sperated in the CSV.
    // Combine them.
    let mut seen_orders: HashMap<String, Order> = HashMap::new();
    for order in &orders[start_from..]
    {
        if let Some(seen_order) = seen_orders.get_mut(&order.order_number)
        {
            *seen_order += order.clone();
        }
        else
        {
            seen_orders.insert(order.order_number.clone(), order.clone());
        }
    }

    let mut orders: Vec<&Order> = seen_orders.values().collect();
    orders.sort_by_key(|o| o.date);
    for order in orders
    {
        println!("Next order:\n");
        println!("{}", order.beancountEntry(&conf));
        println!("\nURL: {}\n", order.url());
        pressAnyKeyToContinue();
    }
    println!("All done.");
    Ok(())
}
