use clap::Parser;

use list::ListSubCommand;
use report::ReportSubCommand;

pub mod list;
pub mod report;

#[derive(Debug, Parser)]
#[command(version)]
pub struct SatrCommand {
    #[clap(subcommand)]
    pub action: Action,
}

impl SatrCommand {
    pub fn action(&self) -> &Action {
        &self.action
    }
}

#[derive(Debug, Parser)]
pub enum Action {
    /// Imprime un reporte de las facturas según los parámetros seleccionados.
    Report(ReportSubCommand),

    /// Lista una breve descripción de las facturas según los parámetros seleccionados.
    Ls(ListSubCommand),

    /// Imprime el contenido de todas las facturas.
    Print,

    /// Imprime el contenido las facturas según los parámetros seleccionados.
    Find,
}

impl Action {
    pub fn run(&self) {
        match self {
            Action::Report(cmd) => cmd.run(),
            Action::Ls(cmd) => cmd.run(),
            _ => println!("Pronto disponible"),
        }
    }
}
