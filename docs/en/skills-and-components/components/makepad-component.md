# Makepad Component

> https://github.com/ZhangHanDong/makepad-component

A modern UI component library for [Makepad](https://github.com/makepad/makepad), inspired by Longbridge's [gpui-component](https://github.com/longbridge/gpui-component).

![Makepad Component Preview](./images/makepad-component.png)

## About Makepad

[Makepad](https://github.com/makepad/makepad) is a next-generation UI framework written in Rust, featuring:

- **GPU-accelerated rendering** - Custom shader-based drawing with SDF (Signed Distance Field)
- **Cross-platform** - Desktop (Windows, macOS, Linux), Mobile (iOS, Android), and Web (WebAssembly)
- **Live design** - Hot-reload DSL for rapid UI iteration
- **High performance** - Designed for demanding applications like IDEs and real-time tools

### Production Applications

| Project | Description |
|---------|-------------|
| [Robrix](https://github.com/project-robius/robrix) | A Matrix chat client built with Makepad |
| [Moly](https://github.com/moxin-org/moly) | AI model manager and inference tool |
| [Makepad Studio](https://github.com/makepad/makepad) | The Makepad IDE itself |

These projects are developed under the [Robius](https://github.com/project-robius) initiative, advancing cross-platform Rust GUI development.

## Features

### Components (v0.1.0)

- **Button** - Primary, Secondary, Danger, Ghost variants with sizes
- **Checkbox** - With label and indeterminate state
- **Switch** - Toggle switch with animations
- **Radio** - Radio button groups
- **Divider** - Horizontal/vertical separators
- **Progress** - Linear progress bar
- **Slider** - Single/Range mode, Vertical, Logarithmic scale, Disabled state
- **Badge** - Notification badges with variants
- **Tooltip** - Four positions with edge detection and auto-flip
- **Input** - Text input field

### Coming Soon

- Spinner
- Modal
- Dropdown
- Select
- And more...

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
makepad-component = { git = "https://github.com/ZhangHanDong/makepad-component", branch = "main" }
```

## Usage

```rust
use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use makepad_component::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                body = <View> {
                    flow: Down, spacing: 20, padding: 20

                    <MpButtonPrimary> { text: "Primary Button" }
                    <MpCheckbox> { text: "Check me" }
                    <MpSwitch> {}
                    <MpSlider> { value: 50.0, min: 0.0, max: 100.0 }
                }
            }
        }
    }
}
```

## Running the Demo

### Desktop

```bash
# Clone the repository
git clone https://github.com/ZhangHanDong/makepad-component
cd makepad-component

# Run the component zoo demo
cargo run -p component-zoo
```

### Web (WebAssembly)

```bash
# Install cargo-makepad (if not installed)
cargo install --force --git https://github.com/makepad/makepad.git --branch rik cargo-makepad

# Install wasm toolchain
cargo makepad wasm install-toolchain

# Build for web
cargo makepad wasm build -p component-zoo --release

# Serve locally (requires Python 3)
python3 serve_wasm.py 8080
# Open http://localhost:8080 in your browser
```

---

## AI-Assisted Development

This component library was built collaboratively with AI (Claude Code) using [makepad-skills](https://github.com/ZhangHanDong/makepad-skills).

makepad-skills is a comprehensive set of Claude Code skills designed for Makepad development, covering widget creation, shader programming, and production-ready patterns.

---

## Inspiration

This project draws inspiration from Longbridge's [gpui-component](https://github.com/longbridge/gpui-component), a component library for the GPUI framework (used in Zed editor). While gpui-component targets GPUI, **makepad-component** brings similar design principles and component patterns to the Makepad ecosystem.

Key differences:
- **Makepad** uses `live_design!` DSL vs GPUI's Rust-only approach
- **Makepad** has built-in shader/animation system
- **Makepad** targets more platforms (including mobile/web)
