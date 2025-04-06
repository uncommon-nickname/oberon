use std::io::Result as IoResult;
use std::sync::Arc;

use oberon::oberon_core::ecs::World;
use oberon::oberon_core::linalg::Point2;
use oberon::oberon_core::rand::rng;
use oberon::oberon_core::style::{Color, Rgb};
use oberon::oberon_core::terminal::Cell;
use oberon::prelude::*;
use oberon_core::rand::Rng;

struct Animation
{
    start_color: Rgb,
    end_color: Rgb,
    end_time: f32,
    elapsed: f32,
}

impl Animation
{
    fn new() -> Self
    {
        let mut rng = rng();
        let start_color = Rgb::BLACK;
        let end_color = Rgb::WHITE;
        let end_time = rng.random_range(2.0..8.0);
        let elapsed = 0.0;

        Self {
            start_color,
            end_color,
            end_time,
            elapsed,
        }
    }

    fn interpolate(&mut self, dt: f32) -> Rgb
    {
        let time_factor = (self.elapsed / self.end_time).clamp(0.0, 1.0);

        if time_factor == 1.0
        {
            self.elapsed = 0.0;
            let old_start = self.start_color;
            self.start_color = self.end_color;
            self.end_color = old_start;

            return self.interpolate(dt);
        }

        let r = self.new_color(time_factor, self.start_color.red(), self.end_color.red());
        let b = self.new_color(time_factor, self.start_color.blue(), self.end_color.blue());
        let g = self.new_color(
            time_factor,
            self.start_color.green(),
            self.end_color.green(),
        );

        self.elapsed += dt;

        Rgb::new(r as u8, g as u8, b as u8)
    }

    fn new_color(&self, time_factor: f32, start_value: u8, end_value: u8) -> f32
    {
        (1.0 - time_factor) * start_value as f32 + time_factor * end_value as f32
    }
}

struct App
{
    world: World,
}

impl ApplicationHandler for App
{
    fn before_start(&mut self, canvas: Canvas<'_>)
    {
        self.world = World::new(canvas.area())
            .register::<Point2>()
            .register::<Animation>();

        let size = canvas.size();

        for x in 0..size.x
        {
            for y in 0..size.y
            {
                let position = Point2::new(x, y);
                let animation = Animation::new();

                self.world
                    .spawn()
                    .with::<Point2>(position)
                    .with::<Animation>(animation);
            }
        }
    }

    fn frame(&mut self, mut canvas: Canvas<'_>, dt: f32, _: &mut Arc<Loop>)
    {
        canvas.erase();

        for id in 0..canvas.area()
        {
            let animation = self.world.get_mut::<Animation>(id).unwrap();
            let next = animation.interpolate(dt);

            let position = self.world.get::<Point2>(id).unwrap();
            let cell = Cell::EMPTY.with_bg(Color::Rgb(next));

            canvas.draw(*position, cell);
        }
    }
}

fn main() -> IoResult<()>
{
    let mut oberon = Oberon::new(Config::default())?;

    let app = App {
        world: World::new(0),
    };

    oberon.run_application(app)
}
