use clap::{Parser, Subcommand};

mod msr;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "ppc64-reg-decoder")]
#[command(about = "A decoder for ppc64 registers", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Decode Machine State Register
    Msr {
        reg_value: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Msr { reg_value } => {
            let msr = msr::Msr::new(reg_value.as_str());
            msr.print();
        }
    }

}
