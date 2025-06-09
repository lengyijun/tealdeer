use anstyle::Color;
use clap::builder::styling::Style;
use pulldown_cmark_mdcat::Theme;

pub fn page_theme() -> Theme {
    Theme {
        code_style: Style::new().fg_color(Some(Color::Ansi(anstyle::AnsiColor::Cyan))),
        html_block_style: Style::new().fg_color(Some(Color::Ansi(anstyle::AnsiColor::Green))),
        inline_html_style: Style::new().fg_color(Some(Color::Ansi(anstyle::AnsiColor::Green))),
        link_style: Style::new().fg_color(Some(Color::Ansi(anstyle::AnsiColor::Yellow))),
        image_link_style: Style::new().fg_color(Some(Color::Ansi(anstyle::AnsiColor::Green))),
        rule_color: Color::Ansi(anstyle::AnsiColor::Green),
        code_block_border_color: Color::Ansi(anstyle::AnsiColor::Green),
        heading_style: Style::new().fg_color(Some(Color::Ansi(anstyle::AnsiColor::Green))),
    }
}
