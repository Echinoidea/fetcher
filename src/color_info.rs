use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn print_colors(sample_text: String, display_brights: bool) {
    let colors = vec![
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::Ansi256(8),  // Bright Black
        Color::Ansi256(9),  // Bright Red
        Color::Ansi256(10), // Bright Green
        Color::Ansi256(11), // Bright Yellow
        Color::Ansi256(12), // Bright Blue
        Color::Ansi256(13), // Bright Magenta
        Color::Ansi256(14), // Bright Cyan
        Color::Ansi256(15), // Bright White
    ];

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for color in colors {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(color)))
            .unwrap();
        if display_brights && color.eq(&Color::White) {
            writeln!(&mut stdout, "{sample_text}").unwrap();
        } else {
            write!(&mut stdout, "{sample_text}").unwrap();
        }
    }

    // Reset the color back to default
    stdout.reset().unwrap();
}
