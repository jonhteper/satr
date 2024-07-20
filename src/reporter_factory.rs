use crate::{
    cli::{ReportType, Subject, SubjectType},
    reporter::{Config, DateConfig, Reporter},
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
                    subject_rfc: emisor_rfc,
                    subject_type: SubjectType::Emisor,
                    dates: DateConfig::from((args.date_start, args.date_end)),
                    report_type: ReportType::from(&report_type),
                }
            }
            Subject::Receptor {
                receptor_rfc,
                report_type,
            } => {
                let args = report_type.args();
                Config {
                    subject_rfc: receptor_rfc,
                    subject_type: SubjectType::Receptor,
                    dates: DateConfig::from((args.date_start, args.date_end)),
                    report_type: ReportType::from(&report_type),
                }
            }
        };

        Reporter::new(config)
    }
}
