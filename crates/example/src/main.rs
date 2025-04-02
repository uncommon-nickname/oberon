use std::io::Result as IoResult;

use oberon::Oberon;
use oberon_core::color::Rgb;
use oberon_core::terminal::Cell;

fn main() -> IoResult<()>
{
    let mut oberon = Oberon::new()?;
    oberon.hide_cursor()?;

    let mut cntr = 0;
    loop
    {
        oberon.frame(|mut canvas| {
            canvas.erase();

            let mut cell = Cell::EMPTY;
            cell.bg = Rgb::new(1, 1, cntr);

            canvas.fill(cell);
        })?;
        cntr = cntr.wrapping_add(1);
    }
}
