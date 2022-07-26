
pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 300;

pub fn draw(drops: &mut Vec<Drop>, frame: &mut [u8]) {
    let pixels = frame.chunks_exact_mut(4);
    for pixel in pixels {
        pixel.copy_from_slice(&[0xd3, 0xb7, 0xf7, 0xff]);
    }

    drops.iter_mut().for_each(|d| d.update());

    for drop in drops {
        let mut drawn = false;
        for y in drop.y..(drop.y + drop.z) {
            for x in drop.x..(drop.x + drop.z/4) {
                if let Ok(i) = (y * (WIDTH as i32) * 4 + x * 4).try_into() {
                    if i < frame.len() {
                        drawn = true;
                        frame[i..i + 4].copy_from_slice(&[207, 100, 219, 0xff]);
                    }
                }
            }
        }
        if !drawn {
            drop.y = -drop.z;
        }
    }
}

pub struct Drop {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Drop {
    fn update(&mut self) {
        self.y += std::cmp::max(self.z/4, 1);
    }
}
