// #[derive(Clone, Copy)]
// enum LifeState {
//     Live,
//     Die,
// }

type LifeState = bool;

pub struct LifeGame {
    pub cell: Vec<Vec<(LifeState, u8)>>,
}

fn add_element_upper(v: u8) -> u8 {
    v + 0x10
    // ((v & 0x20) | ((v & 0x10) << 1) | !(v & 0x10)) | (v & 0x0F)
}

fn add_element_lower(v: u8) -> u8 {
    v + 0x01
    // (v & 0x02) | ((v & 0x01) << 1) | !(v & 0x01) | (v & 0xF0)
}

impl LifeGame {
    pub fn new(x: usize, y: usize) -> LifeGame {
        LifeGame {
            cell: vec![vec![(false, 0); x]; y],
        }
    }

    fn check_cell(&mut self, i: usize, j: usize) {
        let x = self.cell[0].len();
        let y = self.cell.len();
        self.cell[i][j].1 = 0;

        if j >= 1 {
            if i >= 1 && self.cell[i - 1][j - 1].0 {
                self.cell[i][j].1 = add_element_lower(self.cell[i][j].1);
            }
            if self.cell[i][j - 1].0 {
                self.cell[i][j].1 = add_element_lower(self.cell[i][j].1);
            }
            if i + 1 < x && self.cell[i + 1][j - 1].0 {
                self.cell[i][j].1 = add_element_lower(self.cell[i][j].1);
            }
        }

        if i >= 1 && self.cell[i - 1][j].0 {
            self.cell[i][j].1 = add_element_lower(self.cell[i][j].1);
        }
        if i + 1 < x && self.cell[i + 1][j].0 {
            self.cell[i][j].1 = add_element_lower(self.cell[i][j].1);
        }

        if j + 1 < y {
            if i >= 1 && self.cell[i - 1][j + 1].0 {
                self.cell[i][j].1 = add_element_lower(self.cell[i][j].1);
            }
            if self.cell[i][j + 1].0 {
                self.cell[i][j].1 = add_element_lower(self.cell[i][j].1);
            }
            if i + 1 < x && self.cell[i + 1][j + 1].0 {
                self.cell[i][j].1 = add_element_lower(self.cell[i][j].1);
            }
        }
    }

    pub fn click(&mut self, i: usize, j: usize) {
        let x = self.cell[0].len();
        let y = self.cell.len();
        if i >= x || j >= y {
            return;
        }
        self.cell[i][j].0 = !self.cell[i][j].0;
        if j >= 1 {
            if i >= 1 {
                self.check_cell(i - 1, j - 1);
            }
            self.check_cell(i, j - 1);
            if i + 1 < x {
                self.check_cell(i + 1, j - 1);
            }
        }

        if i >= 1 {
            self.check_cell(i - 1, j);
        }
        if i + 1 < x {
            self.check_cell(i + 1, j);
        }

        if j + 1 < y {
            if i >= 1 {
                self.check_cell(i - 1, j + 1);
            }
            self.check_cell(i, j + 1);
            if i + 1 < x {
                self.check_cell(i + 1, j + 1);
            }
        }
    }

    pub fn step(&mut self) {
        let x = self.cell[0].len();
        let y = self.cell.len();
        for j in 0..y {
            for i in 0..x {
                let p = self.cell[i][j].1 & 0b1111;
                self.cell[i][j].1 >>= 4;
                let live = p == 3 || (self.cell[i][j].0 && p == 2);
                if live {
                    if j >= 1 {
                        if i >= 1 {
                            self.cell[i - 1][j - 1].1 =
                                add_element_lower(self.cell[i - 1][j - 1].1);
                        }
                        self.cell[i][j - 1].1 = add_element_lower(self.cell[i][j - 1].1);
                        if i + 1 < x {
                            self.cell[i + 1][j - 1].1 =
                                add_element_lower(self.cell[i + 1][j - 1].1);
                        }
                    }

                    if i >= 1 {
                        self.cell[i - 1][j].1 = add_element_lower(self.cell[i - 1][j].1);
                    }
                    if i + 1 < x {
                        self.cell[i + 1][j].1 = add_element_upper(self.cell[i + 1][j].1);
                    }

                    if j + 1 < y {
                        if i >= 1 {
                            self.cell[i - 1][j + 1].1 =
                                add_element_upper(self.cell[i - 1][j + 1].1);
                        }
                        self.cell[i][j + 1].1 = add_element_upper(self.cell[i][j + 1].1);
                        if i + 1 < x {
                            self.cell[i + 1][j + 1].1 =
                                add_element_upper(self.cell[i + 1][j + 1].1);
                        }
                    }
                }
                self.cell[i][j].0 = live;
            }
        }
    }
}
