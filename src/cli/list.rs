use std::{borrow::Cow, env, path::PathBuf};

use chrono::NaiveDate;
use clap::{Parser, ValueEnum};
use Lotus::Lotus;

use crate::{
    bill_extractor::{self, BillExtractor},
    parse_dec,
};

use super::report;

#[derive(Debug, Parser)]
pub struct ListSubCommand {
    #[arg(value_enum)]
    pub subject: Subject,

    /// RFC del emisor o receptor.
    #[clap(value_name = "RFC")]
    pub rfc: String,

    /// Solo se incluirán facturas desde esta fecha, si no se indica se toma la fecha más antigua existente. Usar formato YYYY-MM-DD.
    #[clap(long, short = 's')]
    pub date_start: Option<NaiveDate>,

    /// Solo se incluirán facturas hasta esta fecha, si no se indica se tomará el día actual. Usar formato YYYY-MM-DD.
    #[clap(long, short = 'e')]
    pub date_end: Option<NaiveDate>,

    #[clap(value_name = "PATH")]
    /// Carpeta desde donde se extraerán recursivamente las facturas.
    pub path: Option<PathBuf>,
}

impl ListSubCommand {
    #[inline]
    fn extractor(&self) -> BillExtractor {
        let config = bill_extractor::Config::init(
            self.rfc.clone(),
            self.subject,
            self.date_start,
            self.date_end,
        );

        BillExtractor::new(Cow::Owned(config))
    }

    #[inline]
    fn path(&self) -> PathBuf {
        self.path
            .clone()
            .unwrap_or_else(|| env::current_dir().expect("Error al leer el directorio actual"))
    }

    pub fn run(&self) {
        let extractor = self.extractor();
        let mut bills = extractor
            .extract_bills(self.path())
            .expect("Error al obtener facturas");
        bills.sort_by(|a, b| a.date.cmp(&b.date));

        let formatter = Lotus::new("$", 2);

        println!("----------------------------------------------------------------");
        println!("Fecha               | Emisor        | Receptor     | Total");
        println!("----------------------------------------------------------------");
        for bill in bills {
            println!(
                "{} | {} | {} | {}",
                bill.date,
                bill.emisor.rfc,
                bill.recipient.rfc,
                formatter.format(parse_dec!(bill.total())),
            );

            for (n, concept) in bill.concepts.list.iter().enumerate() {
                let value = formatter.format(parse_dec!(concept.value));
                println!("  {}.- {} - {}", n + 1, concept.description, value)
            }

            println!();
        }
    }
}

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum Subject {
    /// Lista solo las facturas del emisor.
    Emisor,

    /// Lista solo las facturas del receptor.
    Receptor,
}

impl From<Subject> for report::SubjectType {
    fn from(sub: Subject) -> Self {
        match sub {
            Subject::Emisor => report::SubjectType::Emisor,
            Subject::Receptor => report::SubjectType::Receptor,
        }
    }
}
