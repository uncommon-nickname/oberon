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
use std::sync::Arc;

use oberon::oberon_core::linalg::Point2;
use oberon::oberon_core::style::Color;
use oberon::oberon_core::terminal::Cell;
use oberon::prelude::*;

struct Test;

impl ApplicationHandler for Test
{
    fn setup(&mut self, config: &mut Config)
    {
        config.fps = 30.0;
        config.hide_cursor = true;
    }

    fn frame(&mut self, mut canvas: Canvas<'_>, _dt: f64, app_loop: &mut Arc<Loop>)
    {
        let cell = Cell::EMPTY.with_bg(Color::WHITE);
        let position = Point2::new(10, 10);

        canvas.draw(position, cell);
        app_loop.shutdown();
    }
}

fn main() -> std::io::Result<()>
{
    let app = Test {};
    run_oberon_application(app)
}
```

## Authors

- [Wiktor Nowak](@uncommon-nickname)
