//hw2
use std::{ops, fmt};


#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        let s = values.to_vec();
        let matrix = Matrix {
            col: col,
            row: row,
            data: s,
        };
        return matrix;
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        let vector = Vec::with_capacity(col*row);
        Matrix {
            col: col,
            row: row,
            data: vector,
        }
    }

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        & mut self.data
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        (self.row,self.col)
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row{
            panic!("No")
        }
            else if self.col != rhs.col{
                panic!("No")
            }else {
                let l = self.data;
                let r = rhs.data;
                let mut s = Vec::new();
                let mut done = false;
                let mut x = 0;
                let y = l.len();
                while x < y {
                    let temp = l[x] + r[x];
                    s.push(temp);

                    x = x+1;
                };
                Matrix {
                    col: self.col,
                    row: self.row,
                    data: s,
                }
            }
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row{
            panic!("No")
        }
            else if self.col != rhs.col{
                panic!("No")
            }else {
                let l = self.data;
                let r = rhs.data;
                let mut s = Vec::new();
                let mut done = false;
                let mut x = 0;
                let y = l.len();
                while x < y {
                    let temp = l[x] - r[x];
                    s.push(temp);

                    x = x+1;
                };
                Matrix {
                    col: self.col,
                    row: self.row,
                    data: s,
                }
            }
    }

}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy + fmt::Display > ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.data.to_vec();
        let b = rhs.data.to_vec();
        let col = self.col;
        let rcol = rhs.col;
        let row = rhs.row;
        let mut stack = Vec::new();
        let mut temp:T;
        if self.col != rhs.row{
            panic!("No");
        }else{
            for x in 0..self.row {
                for y in 0..rhs.col {
                    let index = x * col + 0; //
                    let index2= 0 *rcol + y;
                    let mut temp:T = a[index] * b[index2];
                    println!("{}",temp);
                    for z in 1..rhs.row{
                        let index = x * col + z; // 
                        let index2= z *rcol + y;
                        temp = temp + a[index] * b[index2];
                    }
                    stack.push(temp);
                }
            }
        }
        Matrix{
            data: stack,
            row : self.row,
            col : rhs.col,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.data.len();
        let mut s = String::new();
        let a = &self.data;
        let mut count = 0;
            for x in a {
                count+=1;
                if count % self.col == 0
                    {
                        s.push_str(&x.to_string());
                        s.push_str("\n");
                    }
                else{
                    s.push_str(&x.to_string());
                    s.push_str(" ");
                }
            };

        write!(f, "{}",s)
    }
}

