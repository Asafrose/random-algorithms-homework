use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Index, IndexMut, Mul},
};

use anyhow::{Error, Result};

use crate::extensions::vec_extensions::{Single, TryCollect};

#[derive(Debug, Clone)]
pub struct Matrix<TItem> {
    m: usize,
    n: usize,
    inner: Vec<Vec<TItem>>,
}

impl<TItem> Index<(usize, usize)> for Matrix<TItem> {
    type Output = TItem;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.inner[index.0][index.1]
    }
}

impl<TItem> IndexMut<(usize, usize)> for Matrix<TItem> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.inner[index.0][index.1]
    }
}

impl<TItem: Default> Matrix<TItem> {
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            m,
            n,
            inner: (0..m)
                .map(|_| (0..n).map(|_| TItem::default()).collect())
                .collect(),
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = MatrixRow<TItem>> {
        self.inner.iter_mut().map(|vec| MatrixRow { inner: vec })
    }
}

impl<TItem: Default + Mul + Copy> Mul for Matrix<TItem>
where
    TItem::Output: Sum + Default,
{
    type Output = Result<Matrix<TItem::Output>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.n != rhs.m {
            Err(Error::msg(format!(
                "cannot multiply matrixes [left.n={} right.m={}",
                self.n, rhs.m
            )))
        } else {
            let n = rhs.n;
            let m = self.m;
            let mut matrix = Matrix::new(n, m);

            for i in 0..m {
                for j in 0..n {
                    matrix[(i, j)] = (0..m).map(|index| self[(i, index)] * rhs[(index, j)]).sum();
                }
            }

            Ok(matrix)
        }
    }
}

impl<TItem: Default + Mul + Copy> Mul<Vec<TItem>> for Matrix<TItem>
where
    TItem::Output: Default + Sum + Copy,
{
    type Output = Result<Vec<TItem::Output>>;

    fn mul(self, rhs: Vec<TItem>) -> Self::Output {
        (self * (Matrix::from(rhs)))?.try_into()
    }
}

impl<TItem: Mul + Copy> Mul<TItem> for Matrix<TItem>
where
    TItem::Output: Default,
{
    type Output = Matrix<TItem::Output>;

    fn mul(self, rhs: TItem) -> Self::Output {
        let mut matrix = Matrix::new(self.n, self.m);

        for line in matrix.inner.iter_mut().zip(self.inner.iter()) {
            for item in line.0.iter_mut().zip(line.1.iter()) {
                *item.0 = rhs.clone() * *item.1;
            }
        }

        matrix
    }
}

impl<TItem: Display> Display for Matrix<TItem> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = (0..self.m)
            .map(|i| {
                (0..self.n)
                    .map(|j| self[(i, j)].to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .collect::<Vec<String>>();

        let horizontal_delimiter = (0..(lines
            .iter()
            .map(|line| line.len())
            .max()
            .ok_or(std::fmt::Error::default())?))
            .map(|_| "-".into())
            .collect::<Vec<String>>()
            .join("");
        writeln!(f, " {} ", &horizontal_delimiter)?;
        for line in lines {
            writeln!(f, "|{}|", line)?;
        }
        writeln!(f, " {} ", &horizontal_delimiter)?;

        std::fmt::Result::Ok(())
    }
}

impl<TItem> From<Vec<TItem>> for Matrix<TItem> {
    fn from(source: Vec<TItem>) -> Self {
        Matrix {
            m: source.len(),
            n: 1,
            inner: source.into_iter().map(|item| vec![item]).collect(),
        }
    }
}

impl<TItem: Copy> TryInto<Vec<TItem>> for Matrix<TItem> {
    type Error = Error;

    fn try_into(self) -> Result<Vec<TItem>, Self::Error> {
        self.inner.into_iter().map(|vec| vec.single()).try_collect()
    }
}

pub struct MatrixRow<'a, TItem> {
    inner: &'a mut Vec<TItem>,
}

impl<'a, TItem> MatrixRow<'a, TItem> {
    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut TItem> {
        self.inner.iter_mut()
    }
}
