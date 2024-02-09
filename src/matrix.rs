use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T> {
    pub height: u32,
    pub width: u32,
    pub data: Vec<T>,
}

impl<T: fmt::Debug + Ord + Copy> Matrix<T> {
    pub fn new(height: u32, width: u32, data: Vec<T>) -> Matrix<T> {
        Matrix { height, width, data }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
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

    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) {
        let mut new_data = Vec::with_capacity((width * height) as usize);

        for j in y..y+height {
            for i in x..x+width {
                let value = *self.get_value_at(i, j);
                new_data.push(value);
            }
        }

        self.data = new_data;
        self.width = width;
        self.height = height;
    }
}
