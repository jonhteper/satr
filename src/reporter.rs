use std::{borrow::Cow, path::Path};

use rust_decimal::Decimal;

use crate::{
    bill::Bill,
    bill_extractor::{self, BillExtractor},
    cli::report::ReportType,
};

pub struct Reporter {
    pub config: Config,
}

impl Reporter {
    pub fn new(config: Config) -> Self {
        Reporter { config }
    }

    #[inline]
    fn get_result<'a, F>(bills: &'a [Bill], f: F) -> Decimal
    where
        F: Fn(&'a Bill) -> Decimal,
    {
        bills.iter().fold(Decimal::ZERO, |acc, bill| acc + f(bill))
    }

    pub fn money_report<P: AsRef<Path>>(&self, path: P) -> Result<Decimal, String> {
        let config = &self.config.extractor_config;
        let extractor = BillExtractor::new(Cow::Borrowed(config));
        let bills = extractor.extract_bills(path)?;

        let result = match self.config.report_type {
            ReportType::Total => Self::get_result(&bills, Bill::total),
            ReportType::Subtotal => Self::get_result(&bills, Bill::subtotal),
            ReportType::Iva => Self::get_result(&bills, Bill::iva),
            ReportType::Isr => Self::get_result(&bills, Bill::isr),
        };

        Ok(result)
    }
}

pub struct Config {
    pub extractor_config: bill_extractor::Config,
    pub report_type: ReportType,
}
