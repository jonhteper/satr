use clap::Parser;

mod bill;
mod bill_extractor;
mod cli;
mod macros;
mod reporter;
mod reporter_factory;
mod xml_extractor;

#[cfg(test)]
mod tests;

fn main() {
    cli::SatrCommand::parse().action().run();
}
