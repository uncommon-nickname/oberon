use std::io::Result as IoResult;

use oberon::core::linalg::Point2;
use oberon::core::rand::{rng, Rng};
use oberon::core::style::{Color, Rgb};
use oberon::core::terminal::Cell;
use oberon::ecs::World;
use oberon::prelude::*;

struct Animation
{
    start_color: Rgb,
    end_color: Rgb,
    end_time: f64,
    elapsed: f64,
}

impl Animation
{
    fn new() -> Self
    {
        let mut rng = rng();

        let start_color = Rgb::GREEN;
        let end_color = Rgb::BLUE;
        let end_time = rng.random_range(2.0..10.0);
        let elapsed = 0.0;

        Self {
            start_color,
            end_color,
            end_time,
            elapsed,
        }
    }

    fn interpolate(&mut self, dt: f64) -> Rgb
    {
        let time_factor = (self.elapsed / self.end_time).clamp(0.0, 1.0);

        if time_factor == 1.0
        {
            std::mem::swap(&mut self.start_color, &mut self.end_color);
            self.elapsed = 0.0;
            return self.interpolate(dt);
        }

        let new_color = self.start_color.mix(self.end_color, time_factor);
        self.elapsed += dt;

        new_color
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
        self.world = World::new(canvas.area() as usize)
            .register::<Point2>()
            .register::<Animation>();

        let size = canvas.size();

        for x in 0..size.x
        {
            for y in 0..size.y
            {
                self.world
                    .spawn()
                    .with(Point2::from_signed(x, y))
                    .with(Animation::new());
            }
        }
    }

    fn frame(&mut self, mut canvas: Canvas<'_>, dt: f64, _: &mut Arc<Loop>)
    {
        self.world.for_each::<Point2>(|id, pos| {
            let mut animation = self.world.get_mut::<Animation>(id).unwrap();

            let next_color = animation.interpolate(dt);
            let cell = Cell::EMPTY.with_bg(Color::Rgb(next_color));

            canvas.draw(*pos, cell);
        });
    }
}

fn main() -> IoResult<()>
{
    let app = App {
        world: World::new(0),
    };

    run_oberon_application(app)
}
