use std::fmt::{self};

pub struct Matrix<T> {
    pub height: u32,
    pub width: u32,
    pub data: Vec<T>,
}

impl<T: fmt::Debug> Matrix<T> {
    pub fn new(height: u32, width: u32, data: Vec<T>) -> Matrix<T> {
        Matrix { height, width, data }
    }

    pub fn value_at(&self, x: u32, y: u32) -> &T {
        &self.data[(y * self.width + x) as usize]
    }

    pub fn set_value_at(&mut self, x: u32, y: u32, value: T) {
        self.data[(y * self.width + x) as usize] = value;
    }

    pub fn as_csv(&self) -> String {
        let mut buffer = String::with_capacity((self.height * self.width) as usize);

        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.value_at(x, y);

                buffer += &format!("{value:?},")
            }

            buffer += "\n";
        }

        buffer
    }
}
