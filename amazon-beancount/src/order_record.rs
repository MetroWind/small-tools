use chrono::prelude::*;
use csv;

use crate::error::Error;
use crate::config::Config;

// Take a date string like “01/31/21” and return its interpreted local
// date (“2021-01-31”).
fn dateFromDumbSlash(date: &str) -> Result<Date<Local>, Error>
{
    let parts: Vec<&str> = date.split('/').collect();
    Ok(Local.ymd((String::from("20") + parts[2]).parse().map_err(
        |_| rterr!("Failed to parse date"))?,
                 parts[0].parse().map_err(
                     |_| rterr!("Failed to parse date"))?,
                 parts[1].parse().map_err(
                     |_| rterr!("Failed to parse date"))?))
}

// Strip a leading ‘$’ if present.
fn stripDollar(price: &str) -> String
{
    if let Some(s) = price.get(0..1)
    {
        if s == "$"
        {
            if let Some(num_part) = price.get(1..)
            {
                num_part.to_owned()
            }
            else
            {
                String::new()
            }
        }
        else
        {
            price.to_owned()
        }
    }
    else
    {
        String::new()
    }
}

// Convert “$1.23” to float 1.23. Return 0 if string is empty.
fn priceStr2Float(price: &str) -> Result<f64, Error>
{
    let p = stripDollar(price);
    if p.is_empty()
    {
        Ok(0.0)
    }
    else
    {
        p.parse().map_err(|_| rterr!("Failed to parse '{}' into float", price))
    }
}

#[derive(Clone)]
pub struct Order
{
    pub date: Date<Local>,
    pub order_number: String,
    tax: f64,
    shipping: f64,
    sub_total: f64,
}

impl Order
{
    pub fn new() -> Self
    {
        Self {
            date: Local::today(),
            order_number: String::new(),
            tax: 0.0,
            shipping: 0.0,
            sub_total: 0.0,
        }
    }

    fn fromCSVRow(row: csv::StringRecord) -> Result<Self, Error>
    {
        let err = rterr!("Invalid CSV");

        Ok(Self {
            date: dateFromDumbSlash(row.get(0).ok_or_else(|| err.clone())?)?,
            order_number: row.get(1).ok_or_else(|| err.clone())?.to_owned(),
            tax: priceStr2Float(row.get(19).ok_or_else(|| err.clone())?)?,
            shipping: priceStr2Float(row.get(16).ok_or_else(|| err.clone())?)?,
            sub_total: priceStr2Float(row.get(20).ok_or_else(|| err.clone())?)?,
        })
    }

    pub fn fromCSV(filename: &str) -> Result<Vec<Self>, Error>
    {
        // This skips the first line by default.
        let mut csv_reader = csv::Reader::from_path(filename).map_err(
            |_| rterr!("Failed to read CSV file: {}", filename))?;
        csv_reader.records().map(
            |row| Self::fromCSVRow(
                row.map_err(|_| rterr!("Failed to read a CSV row."))?))
            .collect()
    }

    pub fn url(&self) -> String
    {
        format!("https://www.amazon.com/gp/your-account/order-details?orderID={}",
                self.order_number)
    }

    pub fn beancountEntry(&self, config: &Config) -> String
    {
        let mut lines = Vec::new();
        lines.push(
            format!(r#"{} * "Amazon" """#, self.date.format("%F").to_string()));
        lines.push(
            format!("  order-number: {}", self.order_number));
        lines.push(
            format!("  {}", config.account_expense));
        if self.shipping != 0.0
        {
            lines.push(
                format!("  {} {:.2} USD", config.account_shipping,
                        self.shipping));
        }
        if self.tax != 0.0
        {
            lines.push(
                format!("  {} {:.2} USD", config.account_tax, self.tax));
        }
        lines.push(
            format!("  {} -{:.2} USD", config.account_credit, self.sub_total));
        lines.join("\n")
    }
}

impl std::ops::AddAssign for Order
{
    fn add_assign(&mut self, other: Self)
    {
        self.shipping += other.shipping;
        self.sub_total += other.sub_total;
        self.tax += other.tax;
    }
}
