use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version, author, about)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,

    /// optional text
    #[arg(short = 't', long = "text", value_name = "TEXT", default_value = "abc")]
    text: String,
}

#[derive(Subcommand)]
enum SubCommands {
    /// get logs
    #[command(arg_required_else_help = true)]
    Get {
        /// log format
        #[arg(
            short = 'f',
            long = "Format",
            // require_equals = true,
            ignore_case = true
        )]
        #[arg(value_enum)]
        format: Format,
    },

    /// post logs
    Post,
}

#[derive(Clone, ValueEnum)]
enum Format {
    /// csv
    Csv,

    /// json
    Json,
}

fn main() {
    let cli = Cli::parse();

    match cli.subcommand {
        SubCommands::Get { format } => match format {
            Format::Csv => todo!(),
            Format::Json => todo!(),
        },

        SubCommands::Post => {
            todo!()
        }
    }
}
