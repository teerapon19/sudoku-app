use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sun {
    pub x_pos: usize,
    pub y_pos: usize,
    pub bx_pos: usize,
    pub by_pos: usize,
    pub value: usize,
    pub label: usize,
    pub is_freeze: bool,
    pub is_repeat: bool,
}

impl Sun {
    pub fn new(x_pos: usize, y_pos: usize, value: usize, label: usize, level: usize) -> Sun {
        Sun {
            x_pos,
            y_pos,
            value,
            bx_pos: Sun::position(x_pos, level),
            by_pos: Sun::position(y_pos, level),
            label,
            is_freeze: true,
            is_repeat: false,
        }
    }

    fn position(p: usize, level: usize) -> usize {
        let pf = p as f64;
        let levelf = level as f64;
        (pf / levelf).floor() as usize
    }
}
