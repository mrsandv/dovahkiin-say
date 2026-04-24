// ANSI Color codes.
const RED: &str = "\x1b[31m";
const BOLD_WHITE: &str = "\x1b[1;37m";
const RESET: &str = "\x1b[0m";

/// Renders text inside Alduin's scroll.
pub fn render(text: &str) -> String {
    const INNER_WIDTH: usize = 69;
    const TEXT_WIDTH: usize = 65;
    const TEXT_LINES: usize = 4;

    let wrapped = word_wrap(text, TEXT_WIDTH);
    let mut result = String::new();

    result.push_str(RED);
    result.push_str(ALDUIN_TOP);

    for i in 0..TEXT_LINES {
        let content = if i < wrapped.len() {
            let padding_total = INNER_WIDTH - wrapped[i].len();
            let pad_left = padding_total / 2;
            let pad_right = padding_total - pad_left;
            format!(
                "{RED}|{BOLD_WHITE}{}{}{}{RED}|\n",
                " ".repeat(pad_left),
                wrapped[i],
                " ".repeat(pad_right),
            )
        } else {
            format!("{RED}|{}|\n", " ".repeat(INNER_WIDTH))
        };
        result.push_str(&content);
    }

    result.push_str(RED);
    result.push_str(ALDUIN_BOTTOM);
    result.push_str(RESET);
    result
}

/// Splits text into lines within max_width.
fn word_wrap(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();

    for paragraph in text.split('\n') {
        let mut current_line = String::new();
        for word in paragraph.split_whitespace() {
            if current_line.is_empty() {
                current_line.push_str(word);
            } else if current_line.len() + 1 + word.len() > max_width {
                lines.push(current_line);
                current_line = String::from(word);
            } else {
                current_line.push(' ');
                current_line.push_str(word);
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }

    lines
}

// Alduin ASCII art by Alan Greep
// Source: https://asciiart.website/art/1469
const ALDUIN_TOP: &str = r#"                          )       \   /      (
                         /|\      )\_/(     /|\
*                       / | \    (/\|/\)   / | \                      *
|`.____________________/__|__o____\`|'/___o__|__\___________________.'|
|                           '^`    \|/   '^`                          |
|                                   V                                 |
"#;

const ALDUIN_BOTTOM: &str = r#"| ._________________________________________________________________. |
|'               l    /\ /     \\            \ /\   l                `|
*                l  /   V       ))            V   \ l                 *
                 l/            //                  \I
                               V Alan Greep
"#;
