use std::{fs, path::PathBuf};
use clap::Parser;
use maud::{html, Markup, DOCTYPE, PreEscaped};
use pulldown_cmark::{html as markdown_to_html, Options, Parser as MarkdownParser};
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(name = "md2html", about = "Markdown → HTML converter", version, author)]
struct Args {
    /// Input Markdown file
    #[arg(short, long)]
    input: PathBuf,

    /// Output HTML file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn render_html_page(title: &str, content: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { (title) }
                style {
                    "body { font-family: Arial, sans-serif; max-width: 800px; margin: auto; padding: 2em; }"
                }
            }
            body {
                (PreEscaped(content))
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input = fs::read_to_string(&args.input)?;
    let options = Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES;
    let parser = MarkdownParser::new_ext(&input, options);

    let mut html_output = String::new();
    markdown_to_html::push_html(&mut html_output, parser);

    let page = render_html_page("My MD2HTML", &html_output);
    let output_path = args.output.unwrap_or_else(|| args.input.with_extension("html"));
    fs::write(&output_path, page.into_string())?;

    println!("Wrote HTML to {}", output_path.display());
    Ok(())
}
