use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "unid", about = "Unicode box-drawing diagram renderer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Render a diagram from DSL input
    Render {
        /// Read DSL from file
        #[arg(short, long)]
        file: Option<String>,

        /// Read DSL from inline string (comma-separated)
        #[arg(short, long)]
        inline: Option<String>,

        /// Collision mode (overrides DSL declaration)
        #[arg(long, value_enum)]
        collision: Option<CollisionMode>,
    },

    /// List objects in a diagram
    List {
        /// Read DSL from file
        #[arg(short, long)]
        file: Option<String>,

        /// Read DSL from inline string (comma-separated)
        #[arg(short, long)]
        inline: Option<String>,
    },

    /// Show comprehensive usage guide with examples
    Guide,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CollisionMode {
    On,
    Off,
}
