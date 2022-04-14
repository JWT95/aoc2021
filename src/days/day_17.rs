use anyhow::Result;
const LOWER_X: isize = 79;
const UPPER_X: isize = 137;
const LOWER_Y: isize = -176;
const UPPER_Y: isize = -117;

fn velocity_works(x: isize, y: isize) -> bool {
    let mut vec_x = x;
    let mut vec_y = y;
    let (mut pos_x, mut pos_y) = (0, 0);

    let mut max_height = 0;

    let mut hit = false;

    for i in 0..500 {
        pos_x += vec_x;
        pos_y += vec_y;

        if pos_y > max_height {
            max_height = pos_y;
        }

        if vec_x >= 1 {
            vec_x -= 1;
        }
        vec_y -= 1;

        if pos_x >= LOWER_X && pos_x <= UPPER_X && pos_y >= LOWER_Y && pos_y <= UPPER_Y {
            return true;
        }
    }

    return hit;
}

pub fn day_17() -> Result<()> {
    let mut count = 0;
    for x in 0..200 {
        for y in -176..1000 {
            if velocity_works(x, y) {
                count += 1;
            }
        }
    }

    println!("{:?}", count);

    Ok(())
}
