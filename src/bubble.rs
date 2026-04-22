/// Renderiza texto dentro de una burbuja estilo cowsay.
///
/// `text` se recibe como `&str` (referencia prestada — no tomamos ownership).
/// Retornamos `String` (nuevo texto que el caller será dueño de liberar).
pub fn render(text: &str, max_width: usize) -> String {
    let lines = word_wrap(text, max_width);
    let longest = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut result = String::new();

    // Línea superior: " ___ "
    result.push(' ');
    for _ in 0..longest + 2 {
        result.push('_');
    }
    result.push('\n');

    // Contenido con bordes
    for (i, line) in lines.iter().enumerate() {
        let padded = format!("{:<width$}", line, width = longest);

        if lines.len() == 1 {
            // Línea única: < texto >
            result.push_str(&format!("< {} >\n", padded));
        } else if i == 0 {
            // Primera: / texto \
            result.push_str(&format!("/ {} \\\n", padded));
        } else if i == lines.len() - 1 {
            // Última: \ texto /
            result.push_str(&format!("\\ {} /\n", padded));
        } else {
            // Medio: | texto |
            result.push_str(&format!("| {} |\n", padded));
        }
    }

    // Línea inferior: " --- "
    result.push(' ');
    for _ in 0..longest + 2 {
        result.push('-');
    }
    result.push('\n');

    result
}

/// Divide texto en líneas que no excedan `max_width` caracteres.
///
/// `.split_whitespace()` es un iterator — no aloca memoria hasta que
/// iteras sobre él. Lazy evaluation, como generators en Python.
fn word_wrap(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.len() + 1 + word.len() > max_width {
            // La línea actual se mueve al Vec (ownership transfer)
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

    lines
}

// ==========================================
// Unit tests — solo se compilan con `cargo test`
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line_bubble() {
        let result = render("FUS RO DAH!", 40);
        assert!(result.contains("< FUS RO DAH! >"));
    }

    #[test]
    fn multi_line_wraps() {
        let result = render("I used to be an adventurer like you", 20);
        let lines: Vec<&str> = result.lines().collect();
        // Tiene que haber más de 3 líneas (top + content + bottom)
        assert!(lines.len() > 3);
        // Primera línea de contenido empieza con /
        assert!(lines[1].starts_with('/'));
        // Última línea de contenido empieza con \
        assert!(lines[lines.len() - 2].starts_with('\\'));
    }

    #[test]
    fn word_wrap_respects_width() {
        let lines = word_wrap("one two three four five", 10);
        for line in &lines {
            assert!(line.len() <= 10, "Line too long: '{}'", line);
        }
    }
}
