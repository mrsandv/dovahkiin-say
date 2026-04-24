use clap::Parser;

mod art;
mod quotes;

/// CLI arguments for Dovahkiin.
#[derive(Parser)]
#[command(
    name = "dovahkiin",
    about = "FUS RO DAH! Random Skyrim quotes from Alduin's scroll",
    long_about = "FUS RO DAH!\n\n\
        Dovahkiin displays random Skyrim quotes inside Alduin's scroll.\n\
        Each run shows a different quote from guards, NPCs, and legends of Skyrim.\n\n\
        You can also pass your own text with --quote.\n\n\
        ASCII art by Alan Greep (https://asciiart.website/art/1469)"
)]
struct Cli {
    /// Custom text to display (max 260 chars)
    #[arg(short, long)]
    quote: Option<String>,

    /// Show version and credits
    #[arg(short, long)]
    version: bool,
}

/// Max characters allowed to preserve scroll integrity.
const MAX_QUOTE_LEN: usize = 260;

fn main() {
    let cli = Cli::parse();

    if cli.version {
        print!("{}", art::render(&version_text()));
        return;
    }

    let text = cli.quote.unwrap_or_else(|| quotes::random_quote());

    // Truncate if exceeds limit to protect layout.
    let text = if text.chars().count() > MAX_QUOTE_LEN {
        let truncated: String = text.chars().take(MAX_QUOTE_LEN - 3).collect();
        format!("{}...", truncated)
    } else {
        text
    };

    print!("{}", art::render(&text));
}

/// Returns formatted version and author info.
fn version_text() -> String {
    format!(
        "dovahkiin v{}\n\
         By Marco Sandoval (mrsan)\n\
         https://github.com/mrsandv/dovahkiin-say\n\
         https://spacehole.tech",
        env!("CARGO_PKG_VERSION")
    )
}

