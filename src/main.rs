use clap::Parser;

mod art;
mod quotes;

/// Clap maneja --help automáticamente con los doc comments y atributos.
/// Para --version, lo manejamos manual porque queremos mostrarlo
/// dentro del pergamino de Alduin (no el default aburrido de clap).
///
/// `long_about` es lo que se muestra con `--help` (versión larga).
/// `about` es lo que se muestra con `-h` (versión corta).
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
    /// Custom text to display in the scroll (max 260 chars)
    #[arg(short, long)]
    quote: Option<String>,

    /// Show version and credits
    #[arg(short, long)]
    version: bool,
}

/// Máximo de caracteres para --quote.
/// 4 líneas × 65 chars = 260. Más que eso rompe el pergamino.
const MAX_QUOTE_LEN: usize = 260;

fn main() {
    let cli = Cli::parse();

    if cli.version {
        print!("{}", art::render(&version_text()));
        return;
    }

    let text = cli.quote.unwrap_or_else(|| quotes::random_quote());

    // Truncar si excede el límite — protege el diseño del pergamino.
    // `.chars().take(N).collect()` es la forma correcta en Rust.
    // NO usar `&text[..N]` porque podría cortar un carácter UTF-8 por la mitad
    // y causar panic. Rust strings son UTF-8, no ASCII.
    let text = if text.chars().count() > MAX_QUOTE_LEN {
        let truncated: String = text.chars().take(MAX_QUOTE_LEN - 3).collect();
        format!("{}...", truncated)
    } else {
        text
    };

    print!("{}", art::render(&text));
}

fn version_text() -> String {
    format!(
        "dovahkiin v{}\n\
         By Marco Sandoval <marco@example.com>\n\
         https://github.com/marcosandoval/dovahkiin\n\
         https://marcosandoval.dev",
        env!("CARGO_PKG_VERSION")
    )
}
