use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "unid",
    about = "Unicode box-drawing diagram renderer",
    long_about = "Unicode box-drawing diagram renderer.\n\n\
        A text-based alternative to ASCII diagram editors like Monodraw or ASCIIFlow.\n\
        Renders precise Unicode box-drawing diagrams from a simple DSL via stdin."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List objects in a diagram (stdin)
    List,

    /// Lint DSL input for errors and warnings (stdin)
    Lint,

    /// Show comprehensive usage guide with examples
    Guide,
}
