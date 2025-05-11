# Oberon

A very simple terminal renderer allowing you to interact with the terminal cells using escape characters under the hood.

## Examples

- A small example rendering a gif animation in the terminal:

```bash
cargo run --example miku <use_grayscale: true | false>
```

- An example showing the usage of the built-in enitity component system:

```bash
cargo run --example ecs
```

- An example showing the rendered rotating and translating shapes:

```bash
cargo run --example shapes
```

## Usage

The interface is very simple, but a lot of things are currently in wip state, so everything might change.

```rust
use oberon::core::linalg::Point2;
use oberon::core::style::Color;
use oberon::core::terminal::Cell;
use oberon::prelude::*;

struct Test;

impl ApplicationHandler for Test
{
    fn frame(&mut self, mut canvas: Canvas<'_>, _dt: f64, app_loop: &mut ThreadSafeLoop)
    {
        let cell = Cell::EMPTY.bg(Color::WHITE);
        let position = Point2::new(10, 10);

        canvas.draw(position, cell);

        app_loop.shutdown();
    }
}

fn main() -> std::io::Result<()>
{
    let config = Config::new()?.fps(30.0).show_cursor(true);
    Oberon::new(config)?.run(Test)
}
```

## Authors

- [Wiktor Nowak](@uncommon-nickname)
