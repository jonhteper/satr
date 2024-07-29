use crate::{
    bill_extractor::{self, DateConfig},
    cli::report::{ReportType, Subject, SubjectType},
    reporter::{Config, Reporter},
};

pub struct ReporterFactory;

impl ReporterFactory {
    pub fn from_subject(subject: Subject) -> Reporter {
        let config = match subject {
            Subject::Emisor {
                emisor_rfc,
                report_type,
            } => {
                let args = report_type.args();
                Config {
                    report_type: ReportType::from(&report_type),
                    extractor_config: bill_extractor::Config {
                        subject_rfc: emisor_rfc,
                        subject_type: SubjectType::Emisor,
                        dates: DateConfig::from((args.date_start, args.date_end)),
                    },
                }
            }
            Subject::Receptor {
                receptor_rfc,
                report_type,
            } => {
                let args = report_type.args();
                Config {
                    report_type: ReportType::from(&report_type),
                    extractor_config: bill_extractor::Config {
                        subject_rfc: receptor_rfc,
                        subject_type: SubjectType::Receptor,
                        dates: DateConfig::from((args.date_start, args.date_end)),
                    },
                }
            }
        };

        Reporter::new(config)
    }
}
