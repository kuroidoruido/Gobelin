use crate::{Balance, TransactionBucket};
use chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq)]
pub struct GobelinFile {
    pub month: NaiveDate,
    pub transactions: Vec<TransactionBucket>,
    pub tags: Vec<String>,
    pub balance: Vec<Balance>,
}
