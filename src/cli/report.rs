use std::{env, path::PathBuf};

use chrono::NaiveDate;
use clap::Parser;
use enum_kinds::EnumKind;
use Lotus::Lotus;

use crate::reporter_factory::ReporterFactory;

#[derive(Debug, Parser)]
pub struct ReportSubCommand {
    #[clap(subcommand)]
    pub subject: Subject,
}

impl ReportSubCommand {
    pub fn run(&self) {
        let reporter = ReporterFactory::from_subject(self.subject.clone());
        let args = &self.subject.sub_command().args();
        let result = reporter
            .money_report(args.path())
            .expect("Error al realizar cálculo");

        if !args.formatted() {
            println!("{result}");

            return;
        }

        let formatter = Lotus::new("$", 2);
        let n = f64::try_from(result).expect("Error al formatear resultado");
        let formatted_result = formatter.format(n);

        println!("{formatted_result}");
    }
}

#[derive(Debug, Parser, EnumKind, Clone)]
#[enum_kind(SubjectType)]
pub enum Subject {
    /// Busca solo las facturas del emisor.
    Emisor {
        emisor_rfc: String,

        #[clap(subcommand)]
        report_type: ReportTypeSubCommand,
    },
    /// Busca solo las facturas del receptor.
    Receptor {
        receptor_rfc: String,

        #[clap(subcommand)]
        report_type: ReportTypeSubCommand,
    },
}

impl Subject {
    pub fn sub_command(&self) -> &ReportTypeSubCommand {
        match self {
            Subject::Emisor { report_type, .. } | Subject::Receptor { report_type, .. } => {
                report_type
            }
        }
    }
}

#[derive(Debug, Parser, Clone, EnumKind)]
#[enum_kind(ReportType)]
pub enum ReportTypeSubCommand {
    Total(ReportArgs),
    Subtotal(ReportArgs),
    Iva(ReportArgs),
    Isr(ReportArgs),
}

impl ReportTypeSubCommand {
    pub fn args(&self) -> &ReportArgs {
        match self {
            ReportTypeSubCommand::Total(arg) => arg,
            ReportTypeSubCommand::Subtotal(arg) => arg,
            ReportTypeSubCommand::Iva(arg) => arg,
            ReportTypeSubCommand::Isr(arg) => arg,
        }
    }
}

#[derive(Debug, Parser, Clone)]
pub struct ReportArgs {
    #[clap(long, short = 's')]
    /// Solo se incluirán facturas desde esta fecha, si no se indica se toma la fecha más antigua existente. Usar formato YYYY-MM-DD.
    pub date_start: Option<NaiveDate>,

    #[clap(long, short = 'e')]
    /// Solo se incluirán facturas hasta esta fecha, si no se indica se tomará el día actual. Usar formato YYYY-MM-DD.
    pub date_end: Option<NaiveDate>,

    /// Imprime solo el número resultante.
    #[clap(long, short = 'U')]
    pub unformatted: bool,

    #[clap(value_parser)]
    /// Carpeta desde donde se extraerán recursivamente las facturas.
    pub path: Option<PathBuf>,
}

impl ReportArgs {
    pub fn path(&self) -> PathBuf {
        self.path
            .clone()
            .unwrap_or_else(|| env::current_dir().expect("Error al leer el directorio actual"))
    }

    pub fn formatted(&self) -> bool {
        !self.unformatted
    }
}
