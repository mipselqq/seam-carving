use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub height: u32,
    pub width: u32,
    pub data: Vec<T>,
}

impl<T: fmt::Debug + Ord + Copy> Matrix<T> {
    pub fn new(height: u32, width: u32, data: Vec<T>) -> Matrix<T> {
        Matrix { height, width, data }
    }

    pub fn get_value_at(&self, x: u32, y: u32) -> &T {
        &self.data[(y * self.width + x) as usize]
    }

    pub fn set_value_at(&mut self, x: u32, y: u32, value: T) {
        self.data[(y * self.width + x) as usize] = value;
    }

    pub fn min_index_in_row(&self, y: u32) -> u32 {
        let row_start = (y * self.width) as usize;
        let row_end = ((y + 1) * self.width) as usize;
        let row = &self.data[row_start..row_end];
        let (min_index, _) = row.iter().enumerate().min_by_key(|&(_, &val)| val).unwrap();

        min_index as u32
    }

    pub fn as_csv(&self) -> String {
        let mut buffer = String::with_capacity((self.height * self.width) as usize);

        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.get_value_at(x, y);

                buffer += &format!("{value:?},")
            }

            buffer += "\n";
        }

        buffer
    }
}
