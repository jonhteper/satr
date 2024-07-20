use std::path::Path;

use chrono::{Local, NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

use crate::{
    bill::Bill,
    cli::{ReportType, SubjectType},
    xml_extractor::XmlExtractor,
};

pub struct Reporter {
    pub config: Config,
}

impl Reporter {
    pub fn new(config: Config) -> Self {
        Reporter { config }
    }

    #[inline]
    fn rfc_match(&self, bill: &Bill) -> bool {
        match self.config.subject_type {
            SubjectType::Emisor => bill.emisor.rfc == self.config.subject_rfc,
            SubjectType::Receptor => bill.recipient.rfc == self.config.subject_rfc,
        }
    }

    #[inline]
    fn dates_match(&self, bill: &Bill) -> bool {
        bill.date >= self.config.dates.date_start && bill.date <= self.config.dates.date_end
    }

    #[inline]
    fn filter(&self, bill: Bill) -> Option<Bill> {
        if !self.rfc_match(&bill) {
            return None;
        }

        if !self.dates_match(&bill) {
            return None;
        }

        Some(bill)
    }

    #[inline]
    pub fn extract_bills<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Bill>, String> {
        let files_content = XmlExtractor::load_all_xmls(path)?;
        let bills = files_content
            .iter()
            .map(|file| quick_xml::de::from_str(file))
            .filter_map(Result::ok)
            .filter_map(|b| self.filter(b))
            .collect();

        Ok(bills)
    }

    #[inline]
    fn get_result<'a, F>(bills: &'a [Bill], f: F) -> Decimal
    where
        F: Fn(&'a Bill) -> Decimal,
    {
        bills.iter().fold(Decimal::ZERO, |acc, bill| acc + f(bill))
    }

    pub fn money_report<P: AsRef<Path>>(&self, path: P) -> Result<Decimal, String> {
        let bills = self.extract_bills(path)?;

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
    pub subject_rfc: String,
    pub subject_type: SubjectType,
    pub dates: DateConfig,
    pub report_type: ReportType,
}

pub struct DateConfig {
    pub date_start: NaiveDateTime,
    pub date_end: NaiveDateTime,
}

impl From<(Option<NaiveDate>, Option<NaiveDate>)> for DateConfig {
    fn from((start, end): (Option<NaiveDate>, Option<NaiveDate>)) -> Self {
        let date_start = start.unwrap_or_else(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());
        let date_end = end.unwrap_or_else(|| Local::now().date_naive());

        DateConfig {
            date_start: NaiveDateTime::from(date_start),
            date_end: NaiveDateTime::from(date_end),
        }
    }
}
