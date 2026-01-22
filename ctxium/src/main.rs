use clap::{Parser, Subcommand};

mod bootstrap;
mod hook;
mod hookserver;

#[derive(Parser)]
#[command(name = "ctxium")]
#[command(about = "Context management CLI for AI coding agents")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send hook data to the hookserver (called by Claude Code hooks)
    Hook {
        /// The hook name (e.g., SessionStart, PreToolUse, PostToolUse)
        hook_name: String,
    },

    /// Start the hook server to receive and display hook data
    Hookserver {
        /// Port to listen on
        #[arg(short, long, default_value = "6160")]
        port: u16,

        /// Log file path
        #[arg(short, long, default_value = "~/.contextium/hooks.log")]
        log_file: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hook { hook_name } => {
            // Silent failure - exit code 0 even on error
            let _ = hook::run(&hook_name).await;
        }
        Commands::Hookserver { port, log_file } => {
            if let Err(e) = hookserver::run(port, &log_file).await {
                eprintln!("Hookserver error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
