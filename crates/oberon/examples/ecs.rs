use std::io::Result as IoResult;

use oberon::core::linalg::{Point2, Vec2};
use oberon::core::rand::{rng, Rng};
use oberon::core::style::{Color, Rgb};
use oberon::core::terminal::Cell;
use oberon::ecs::World;
use oberon::prelude::*;

struct Animation
{
    start_color: Rgb,
    end_color: Rgb,
    duration_s: f64,
    elapsed: f64,
}

impl Animation
{
    fn new() -> Self
    {
        let mut rng = rng();

        Self {
            start_color: Rgb::GREEN,
            end_color: Rgb::BLUE,
            duration_s: rng.random_range(2.0..10.0),
            elapsed: 0.0,
        }
    }

    fn interpolate(&mut self, dt: f64) -> Color
    {
        let time_factor = self.elapsed / self.duration_s;

        if time_factor >= 1.0
        {
            std::mem::swap(&mut self.start_color, &mut self.end_color);
            self.elapsed = 0.0;
            return self.interpolate(dt);
        }
        let mixed = self.start_color.mix(self.end_color, time_factor);
        self.elapsed += dt;

        Color::Rgb(mixed)
    }
}

struct App
{
    world: World,
}

impl App
{
    fn new(size: Vec2) -> Self
    {
        let area = (size.x * size.y) as usize;
        let world = World::new(area)
            .register::<Point2>()
            .register::<Animation>();

        Self { world }
    }
}

impl ApplicationHandler for App
{
    fn before_start(&mut self, canvas: Canvas<'_>)
    {
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

    fn frame(&mut self, mut canvas: Canvas<'_>, dt: f64, _: &mut ThreadSafeLoop)
    {
        self.world.for_each::<Point2>(|id, pos| {
            let mut animation = self.world.get_mut::<Animation>(id).unwrap();

            let next_color = animation.interpolate(dt);
            let cell = Cell::EMPTY.bg(next_color);

            canvas.draw(*pos, cell);
        });
    }
}

fn main() -> IoResult<()>
{
    let config = Config::new()?;
    let app = App::new(config.size);

    Oberon::new(config)?.run(app)
}
