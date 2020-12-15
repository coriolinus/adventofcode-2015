use aoc2015::geometry::Point;
use rand::Rng as _;
use std::time::Duration;
use std::{convert::TryFrom, path::Path};

use super::{next_state, Error, Grid, ITERATIONS};

pub const FRAME_DURATION: Duration = Duration::from_millis(200);

fn set_lit_point(position: Point, subpixels: &mut [u8], width: usize) {
    // each lit point illuminates 5 pixels in the shape of a cross, plus
    // up to 4 more, chosen randomly, which form a sparkling effect

    let mut rng = rand::thread_rng();

    const WARM_WHITE: [u8; 3] = [253, 244, 220];

    let x = |point: Point| point.x as usize;
    let y = |point: Point| point.y as usize;

    let row_pixels = pixel_width(width) as usize;

    // the linear index of a position has the following components:
    //
    // - 2: offset from left edge
    // - 2 * row_pixels: offset from top
    // - x(position) * 4: x component of position
    // - y(position) * 4 * row_pixels: y component of position
    // - x(offset): x offset
    // - y(offset) * row_pixels: y offset
    //
    // It is multiplied by 3, because that is how many bytes each pixel takes
    //
    // Note: this requires that the offset be in the positive quadrant
    let linear_idx = |offset: Point| {
        (2 + (2 * row_pixels)
            + (x(position) * 4)
            + (y(position) * 4 * row_pixels)
            + x(offset)
            + (y(offset) * row_pixels))
            * 3
    };

    // central cross shape
    for offset in [
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(2, 1),
        Point::new(1, 2),
    ]
    .iter()
    {
        let idx = linear_idx(*offset);
        subpixels[idx..idx + 3].copy_from_slice(&WARM_WHITE);
    }

    // corners
    for offset in [
        Point::new(0, 0),
        Point::new(0, 2),
        Point::new(2, 0),
        Point::new(2, 2),
    ]
    .iter()
    {
        if rng.gen::<bool>() {
            let idx = linear_idx(*offset);
            subpixels[idx..idx + 3].copy_from_slice(&WARM_WHITE);
        }
    }
}

fn create_frame_from(grid: &Grid) -> gif::Frame {
    // 16 pixels per light: 3x3 with a 1px margin
    // 3 subpixels per pixel; 1 each for r, g, b
    let width = grid.width();
    let mut subpixels = vec![0; n_pixels_for(grid.width(), grid.height()) * 3];
    grid.for_each_point(|light, position| {
        if light.is_on() {
            set_lit_point(position, &mut subpixels, width);
        }
    });
    gif::Frame::from_rgb(pixel_width(width), pixel_height(grid.height()), &subpixels)
}

// each light is 4px wide, with a 2px margin on either side
fn pixel_width(width: usize) -> u16 {
    ((width + 1) * 4) as u16
}

// each light is 4px high, with a 2px margin on either side
fn pixel_height(height: usize) -> u16 {
    ((height + 1) * 4) as u16
}

// total pixels
fn n_pixels_for(width: usize, height: usize) -> usize {
    pixel_width(width) as usize * pixel_height(height) as usize
}

pub fn animate(input: &Path, output: &Path) -> Result<(), Error> {
    let mut grid = Grid::try_from(input)?;
    let output = std::fs::File::create(output)?;
    let output = std::io::BufWriter::new(output);
    let mut output = gif::Encoder::new(
        output,
        pixel_width(grid.width()),
        pixel_height(grid.height()),
        &[],
    )?;

    // configure
    output.set_repeat(gif::Repeat::Infinite)?;
    // note: delay is in hundredths of a second
    output.write_extension(gif::ExtensionData::new_control_ext(
        (FRAME_DURATION.as_millis() / 10) as u16,
        gif::DisposalMethod::Any,
        false,
        None,
    ))?;

    // repeat the initial frame
    // regenerate it each time to preserve wibbliness
    for _ in 0..5 {
        output.write_frame(&create_frame_from(&grid))?;
    }

    // animate
    for _ in 0..ITERATIONS {
        grid = next_state(&grid);
        output.write_frame(&create_frame_from(&grid))?;
    }

    // repeate the final frame 5 more times
    for _ in 0..10 {
        output.write_frame(&create_frame_from(&grid))?;
    }

    Ok(())
}
