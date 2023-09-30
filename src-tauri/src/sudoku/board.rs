use crate::sudoku::sun::*;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    pub level: usize,
    pub size: usize,
    pub item_amount: usize,
    pub item: Vec<Vec<Sun>>,
}

impl Board {
    pub fn new(level: usize) -> Board {
        let size = level.pow(2);
        let mut label = 0;
        let item: Vec<Vec<Sun>> = (0..size)
            .map(|i| {
                (0..size)
                    .map(|j| {
                        let sun = Sun::new(i, j, 0, label, level);
                        label += 1;
                        sun
                    })
                    .collect()
            })
            .collect();
        Board {
            level,
            size,
            item_amount: size * size,
            item,
        }
    }

    pub fn get_level(&self) -> usize {
        self.level
    }

    pub fn get_item(&self) -> Vec<Vec<Sun>> {
        self.item.to_vec()
    }

    pub fn set_item_at(&mut self, x_pos: usize, y_pos: usize, value: usize) {
        if !self.item[x_pos][y_pos].is_freeze {
            self.item[x_pos][y_pos].value = value;
        }
    }

    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let mut is_repeat = false;
        for i in 0..self.size {
            loop {
                let mut nums: Vec<usize> = (1..self.size + 1).collect();
                nums.shuffle(&mut rng);
                for (j, rnum) in nums.iter().enumerate() {
                    is_repeat = self.check_repeat_gen(
                        *rnum,
                        self.item[i][j].bx_pos,
                        self.item[i][j].by_pos,
                        i,
                        j,
                    );
                    if is_repeat {
                        break;
                    }
                }
                if is_repeat {
                    is_repeat = false;
                    continue;
                }
                for (j, rnum) in nums.iter().enumerate() {
                    self.item[i][j].value = *rnum;
                }
                break;
            }
        }
    }

    pub fn hide_sun(&mut self, difficulty_percentage: usize) {
        let amount =
            ((self.item_amount as f64) * (difficulty_percentage as f64) / 100.0).floor() as usize;
        if amount == 0 {
            return;
        }
        let mut rng = rand::thread_rng();
        let mut cut_out_item: Vec<usize> = Vec::new();

        loop {
            let num = rng.gen_range(0..self.item_amount);
            if !cut_out_item.contains(&num) {
                cut_out_item.push(num);
            }
            if amount == cut_out_item.len() {
                break;
            }
        }

        for i in 0..self.size {
            for j in 0..self.size {
                if cut_out_item.contains(&self.item[i][j].label) {
                    self.item[i][j].value = 0;
                    self.item[i][j].is_freeze = false;
                }
            }
        }
    }

    pub fn is_correct_all(&self) -> bool {
        let expect_sum = self.size * (self.size + 1) / 2;
        for i in 0..self.size {
            let mut sum_col: usize = 0;
            let mut sum_row: usize = 0;
            for j in 0..self.size {
                sum_row += self.item[i][j].value;
                sum_col += self.item[j][i].value;
            }
            if sum_row != expect_sum || sum_col != expect_sum {
                return false;
            }
        }
        true
    }

    pub fn clear_repeat_flag(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                self.item[i][j].is_repeat = false;
            }
        }
    }

    pub fn check_repeat_at(&mut self, x_pos: usize, y_pos: usize) {
        let mut is_repeat = false;
        for i in 0..self.size {
            for j in 0..self.size {
                self.item[i][j].is_repeat = false;

                if self.item[i][j].bx_pos == self.item[x_pos][y_pos].bx_pos
                    && self.item[i][j].by_pos == self.item[x_pos][y_pos].by_pos
                    && i != x_pos
                    && j != y_pos
                {
                    if self.item[i][j].value == self.item[x_pos][y_pos].value {
                        is_repeat = true;
                        break;
                    }
                }

                if self.item[x_pos][y_pos].value == self.item[x_pos][j].value && j != y_pos {
                    is_repeat = true;
                    break;
                }
            }
            if is_repeat {
                break;
            }
            if self.item[i][y_pos].value == self.item[x_pos][y_pos].value && i != x_pos {
                is_repeat = true;
                break;
            }
        }
        self.item[x_pos][y_pos].is_repeat = is_repeat
    }

    fn check_repeat_gen(
        &self,
        value: usize,
        bx_pos: usize,
        by_pos: usize,
        row: usize,
        col_position: usize,
    ) -> bool {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.item[i][j].bx_pos == bx_pos && self.item[i][j].by_pos == by_pos {
                    if self.item[i][j].value == value {
                        return true;
                    }
                }
            }
            if self.item[i][col_position].value == value {
                return true;
            }
            if value == self.item[row][i].value {
                return true;
            }
        }
        false
    }
}
