use std::{borrow::Cow, path::Path};

use chrono::{Local, NaiveDate, NaiveDateTime};

use crate::{bill::Bill, cli::report::SubjectType, xml_extractor::XmlExtractor};

#[derive(Debug, Clone)]
pub struct BillExtractor<'a> {
    config: Cow<'a, Config>,
}

impl<'a> BillExtractor<'a> {
    pub fn new(config: Cow<'a, Config>) -> Self {
        BillExtractor { config }
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

    pub fn extract_bills<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Bill>, String> {
        Ok(self.extract_as_iter(path)?.collect())
    }

    pub fn extract_as_iter<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<impl Iterator<Item = Bill> + '_, String> {
        let files_content = XmlExtractor::load_all_xmls(path)?;

        let iter = files_content
            .into_iter()
            .map(|file| quick_xml::de::from_str(file.as_str()))
            .filter_map(Result::ok)
            .filter_map(|b| self.filter(b));

        Ok(iter)
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub subject_rfc: String,
    pub subject_type: SubjectType,
    pub dates: DateConfig,
}

impl Config {
    pub fn init<S: Into<SubjectType>>(
        rfc: String,
        subject: S,
        date_start: Option<NaiveDate>,
        date_end: Option<NaiveDate>,
    ) -> Self {
        Config {
            subject_rfc: rfc,
            subject_type: subject.into(),
            dates: DateConfig::from((date_start, date_end)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DateConfig {
    pub date_start: NaiveDateTime,
    pub date_end: NaiveDateTime,
}

impl From<(Option<NaiveDate>, Option<NaiveDate>)> for DateConfig {
    fn from((start, end): (Option<NaiveDate>, Option<NaiveDate>)) -> Self {
        let date_start = start.unwrap_or_else(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());
        let date_end = match end {
            Some(d) => NaiveDateTime::from(d),
            None => Local::now().naive_local(),
        };

        DateConfig {
            date_start: NaiveDateTime::from(date_start),
            date_end,
        }
    }
}
