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

pub struct Order
{
    date: Date<Local>,
    pub order_number: String,
    tax: String,
    shipping: String,
    sub_total: String,
}

impl Order
{
    fn fromCSVRow(row: csv::StringRecord) -> Result<Self, Error>
    {
        let err = rterr!("Invalid CSV");
        Ok(Self {
            date: dateFromDumbSlash(row.get(0).ok_or_else(|| err.clone())?)?,
            order_number: row.get(1).ok_or_else(|| err.clone())?.to_owned(),
            tax: stripDollar(row.get(19).ok_or_else(|| err.clone())?),
            shipping: stripDollar(row.get(16).ok_or_else(|| err.clone())?),
            sub_total: stripDollar(row.get(20).ok_or_else(|| err.clone())?),
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
        if self.shipping != "0.00"
        {
            lines.push(
                format!("  {} {} USD", config.account_shipping, self.shipping));
        }
        if self.tax != "0.00"
        {
            lines.push(
                format!("  {} {} USD", config.account_tax, self.tax));
        }
        lines.push(
            format!("  {} -{} USD", config.account_credit, self.sub_total));
        lines.join("\n")
    }
}
