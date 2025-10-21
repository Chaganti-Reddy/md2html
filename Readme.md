# 🦀 Rust CLI MD2HTML

A fast, minimal **Markdown → HTML converter** written in pure **Rust**, using:
- [`clap`](https://crates.io/crates/clap) for elegant CLI argument parsing
- [`pulldown-cmark`](https://crates.io/crates/pulldown-cmark) for Markdown parsing
- [`maud`](https://crates.io/crates/maud) for HTML templating

---

## Features

- [x] Converts any `.md` file into a clean `.html` file  
- [x] Supports Markdown tables, strikethrough, and basic GFM syntax  
- [x] Automatically generates an output `.html` file if not specified  
- [x] Fast, zero-dependency binary — no runtime setup needed  
- [x] Simple CLI with `-i` and `-o` flags

---

## Installation

Make sure you have **Rust (>=1.84, Edition 2024)** installed.

```bash
git clone https://github.com/Chaganti-Reddy/md2html.git
cd md2html
cargo build --release
```

The compiled binary will be in:

```bash
./target/release/md2html
```

## Usage

Convert a Markdown file:

```bash
cargo run -- -i test.md
```

this will generate a `test.html` file in the current directory. Convert a Markdown file and specify an output file:

```bash
cargo run -- -i test.md -o test.html
```

Run the CLI with `-h` or `--help` to see all available options.

## Example

Input: `test.md`

```md
# Hello World

This is a *Markdown* **test**.

- Supports lists
- Tables
- Strikethrough ~~like this~~
```

Output: `test.html`

```html
<html>
  <head>
    <title>My MD2HTML</title>
  </head>
  <body>
    <h1>Hello World</h1>
    <p>This is a <em>Markdown</em> <strong>test</strong>.</p>
    <ul>
      <li>Supports lists</li>
      <li>Tables</li>
      <li>Strikethrough <del>like this</del></li>
    </ul>
  </body>
</html>
```

## Code Overview

Main Components:

- `Args` struct: Handles CLI arguments and options via [`clap`](https://crates.io/crates/clap)
- `render_html_page` function: Renders the HTML page using [`maud`](https://crates.io/crates/maud)
- `main` function: Parses Markdown, converts to HTML, and writes the output to a file

## Future Improvements

- [ ] Support more Markdown features
- [ ] Add `--serve` flag for live preview
- [ ] Add support for code blocks and syntax highlighting
- [ ] Add support custom CSS themes 

## Contributing

Pull requests and ideas are welcome!

If you'd like to contribute: 

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Submit a pull request

## License

This project is licensed under the [MIT License](LICENSE).