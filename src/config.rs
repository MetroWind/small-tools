pub struct Config
{
    pub account_credit: String,
    pub account_tax: String,
    pub account_shipping: String,
    pub account_expense: String,
}

impl Default for Config
{
    fn default() -> Self
    {
        Self {
            account_credit: "Liabilities:Credit:Freedom".to_owned(),
            account_tax: "Expenses:Taxes:Consumer".to_owned(),
            account_shipping: "Expenses:Shipping".to_owned(),
            account_expense: "Expenses:Misc".to_owned(),
        }
    }
}
