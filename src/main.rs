use clap::Parser;
use colored::Colorize;

/// Simple utility for printing a message with a newline
#[derive(Debug, Parser)]
#[command(name = "println")]
struct Cli {
    /// The message to print
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Parser)]
enum Commands {
    /// Print a message in black
    #[command(arg_required_else_help = true)]
    Black {
        /// The message to print
        msg: String,
    },

    /// Print a message in blue
    #[command(arg_required_else_help = true)]
    Blue {
        /// The message to print
        msg: String,
    },

    /// Print a message in cyan
    #[command(arg_required_else_help = true)]
    Cyan {
        /// The message to print
        msg: String,
    },

    /// Print a message in magenta
    #[command(arg_required_else_help = true)]
    Magenta {
        /// The message to print
        msg: String,
    },

    /// Print a message
    #[command(arg_required_else_help = true)]
    Msg {
        /// The message to print
        msg: String,
    },

    /// Print a message in green
    #[command(arg_required_else_help = true)]
    Green {
        /// The message to print
        msg: String,
    },

    /// Print a message in red
    #[command(arg_required_else_help = true)]
    Red {
        /// The message to print
        msg: String,
    },

    /// Print a message in yellow
    #[command(arg_required_else_help = true)]
    Yellow {
        /// The message to print
        msg: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Black { msg } => println!("{}", msg.black()),
        Commands::Blue { msg } => println!("{}", msg.blue()),
        Commands::Cyan { msg } => println!("{}", msg.cyan()),
        Commands::Magenta { msg } => println!("{}", msg.magenta()),
        Commands::Msg { msg } => println!("{}", msg),
        Commands::Green { msg } => println!("{}", msg.green()),
        Commands::Red { msg } => println!("{}", msg.red()),
        Commands::Yellow { msg } => println!("{}", msg.yellow()),
    }
}
