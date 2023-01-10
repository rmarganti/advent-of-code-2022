pub struct Grid {
    data: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(data: &str) -> Self {
        let data: Vec<Vec<u8>> = data
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        let width = data[0].len();
        let height = data.len();

        Self {
            data,
            width,
            height,
        }
    }

    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        // Trees along the edges are visible.
        if (x == 0 || x == self.width - 1) || (y == 0 || y == self.height) {
            return true;
        }

        let value = self.data[y][x];

        // Left
        let mut visible_left = true;
        for x_compare in 0..x {
            let value_compare = self.data[y][x_compare];

            if value_compare >= value {
                visible_left = false;
                break;
            }
        }

        // Top
        let mut visible_top = true;
        for y_compare in 0..y {
            let value_compare = self.data[y_compare][x];

            if value_compare >= value {
                visible_top = false;
                break;
            }
        }

        // Right
        let mut visible_right = true;
        for x_compare in (x + 1)..self.width {
            let value_compare = self.data[y][x_compare];

            if value_compare >= value {
                visible_right = false;
                break;
            }
        }

        // Bottom
        let mut visible_bottom = true;
        for y_compare in (y + 1)..self.height {
            let value_compare = self.data[y_compare][x];

            if value_compare >= value {
                visible_bottom = false;
                break;
            }
        }

        return visible_left || visible_top || visible_right || visible_bottom;
    }

    pub fn scenic_score(&self, x: usize, y: usize) -> usize {
        let value = self.data[y][x];

        // ------------------------------------------------
        // Left
        // ------------------------------------------------

        let mut scenic_score_left = 0;

        if x != 0 {
            for x_compare in (0..x).rev() {
                scenic_score_left += 1;

                let value_compare = self.data[y][x_compare];
                if value_compare >= value {
                    break;
                }
            }
        }

        // ------------------------------------------------
        // Top
        // ------------------------------------------------

        let mut scenic_score_top = 0;

        if y != 0 {
            for y_compare in (0..y).rev() {
                scenic_score_top += 1;

                let value_compare = self.data[y_compare][x];
                if value_compare >= value {
                    break;
                }
            }
        }

        // ------------------------------------------------
        // Right
        // ------------------------------------------------

        let mut scenic_score_right = 0;

        if x != self.width - 1 {
            for x_compare in (x + 1)..self.width {
                scenic_score_right += 1;

                let value_compare = self.data[y][x_compare];
                if value_compare >= value {
                    break;
                }
            }
        }

        // ------------------------------------------------
        // Bottom
        // ------------------------------------------------

        let mut scenic_score_bottom = 0;

        if y != self.height - 1 {
            for y_compare in (y + 1)..self.height {
                scenic_score_bottom += 1;

                let value_compare = self.data[y_compare][x];
                if value_compare >= value {
                    break;
                }
            }
        }

        return scenic_score_left * scenic_score_top * scenic_score_right * scenic_score_bottom;
    }
}

#[cfg(test)]
mod tests {
    const TEST_GRID: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn it_determines_if_a_point_is_visible() {
        let grid = super::Grid::new(TEST_GRID);

        assert_eq!(true, grid.is_visible(1, 1));
        assert_eq!(true, grid.is_visible(2, 1));
        assert_eq!(false, grid.is_visible(3, 1));
        assert_eq!(true, grid.is_visible(1, 2));
        assert_eq!(false, grid.is_visible(2, 2));
        assert_eq!(true, grid.is_visible(3, 2));
        assert_eq!(true, grid.is_visible(4, 2));
    }

    #[test]
    fn it_calculates_the_scenic_score() {
        let grid = super::Grid::new(TEST_GRID);

        assert_eq!(4, grid.scenic_score(2, 1));
        assert_eq!(8, grid.scenic_score(2, 3));
    }
}
