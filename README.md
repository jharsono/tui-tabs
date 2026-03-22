# tui-tabs

A tab navigation widget for [Ratatui](https://ratatui.rs) with individually bordered boxes and rounded corners.

![demo](https://raw.githubusercontent.com/jharsono/tui-tabs/main/screenshots/demo.gif)

## Features

- Each tab renders as a bordered box with configurable corner style (rounded or square)
- Active tab opens into the content below via junction corners
- Continuous baseline spans the full widget width
- Optional indicator symbol next to the active tab label
- Builder API following Ratatui conventions

## Installation

```bash
cargo add tui-tabs
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
tui-tabs = "0.1"
```

## Usage

```rust
use ratatui::style::{Color, Style};
use tui_tabs::TabNav;

let widget = TabNav::new(&["Files", "Search", "Settings"], 0)
    .highlight_style(Style::new().fg(Color::Cyan))
    .border_style(Style::new().fg(Color::DarkGray));
```

The widget requires exactly 3 rows of height (top border, label row, baseline).

## Builder Methods

| Method | Default | Description |
|--------|---------|-------------|
| `style()` | Unstyled | Inactive tab label style |
| `highlight_style()` | Unstyled | Active tab label style |
| `highlight_bold()` | `true` | Auto-apply bold to active tab |
| `border_style()` | Unstyled | Border and baseline style |
| `indicator()` | `Some("▸")` | Symbol left of active label; `None` to disable |
| `border_set()` | `ROUNDED` | Border character set (`ROUNDED`, `PLAIN`, etc.) |

## Examples

```bash
cargo run --example basic        # Static render, press any key to exit
cargo run --example interactive  # Arrow keys to navigate, q to quit
cargo run --example styled       # Square borders, custom colors, no indicator
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
