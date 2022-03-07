use paste::paste;

use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
    SubAssign,
};

pub trait TList<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;

    fn set(&mut self, val: T);
}

macro_rules! impl_op_insides {
    ($fun_name: ident) => {
        type Output = Self;

        fn $fun_name(self, rhs: T) -> Self::Output {
            let mut res = self;

            for val in res.0.iter_mut() {
                paste! {
                    (*val).[< $fun_name _assign >](rhs);
                }
            }

            res
        }
    };

    ($fun_name: ident, assign) => {
        paste! {
            fn [< $fun_name _assign >](&mut self, rhs: T) {
                for val in self.0.iter_mut() {
                    (*val).[< $fun_name _assign >](rhs);
                }
            }
        }
    };
}

macro_rules! impl_op {
    (array; $($op_name:ident, $fun_name:ident);*) => {
        paste! {
            $(
                impl<T: [< $op_name Assign >] + Copy, const N: usize> $op_name<T> for TArray<T, N> {
                    impl_op_insides!($fun_name);
                }

                impl<T: [< $op_name Assign >] + Copy, const N: usize> [< $op_name Assign>]<T> for TArray<T, N> {
                    impl_op_insides!($fun_name, assign);
                }

                impl<T: [< $op_name Assign >] + Copy, const N: usize> $op_name<TArray<T, N>> for TArray<T, N> {
                    type Output = Self;

                    fn $fun_name(self, rhs: TArray<T, N>) -> Self::Output {
                        let mut res = self;

                        for (index, val) in res.0.iter_mut().enumerate() {
                            (*val).[< $fun_name _assign >](rhs.0[index]);
                        }

                        res
                    }
                }

                impl<T: [< $op_name Assign >] + Copy, const N: usize> [< $op_name Assign>]<TArray<T, N>> for TArray<T, N> {
                    fn [< $fun_name _assign >](&mut self, rhs: TArray<T, N>) {
                        for (index, val) in self.0.iter_mut().enumerate() {
                            (*val).[< $fun_name _assign >](rhs.0[index]);
                        }
                    }
                }
            )*
        }
    };

    (vector; $($op_name:ident, $fun_name:ident);*) => {
        paste! {
            $(
                impl<T: [< $op_name Assign >] + Copy> $op_name<T> for TVector<T> {
                    impl_op_insides!($fun_name);
                }

                impl<T: [< $op_name Assign >] + Copy> [< $op_name Assign>]<T> for TVector<T> {
                    impl_op_insides!($fun_name, assign);
                }

                impl<T: $op_name<Output = T> + Copy> $op_name<TVector<T>> for TVector<T> {
                    type Output = Self;

                    fn $fun_name(self, rhs: TVector<T>) -> Self::Output {
                        let mut res = vec![];

                        for i in 0..self.0.len().min(rhs.0.len()) {
                            res.push(self.0[i].$fun_name(rhs.0[i]));
                        }

                        TVector(res)
                    }
                }

                impl<T: [< $op_name Assign >] + Copy> [< $op_name Assign>]<TVector<T>> for TVector<T> {
                    fn [< $fun_name _assign >](&mut self, rhs: TVector<T>) {
                        for i in 0..self.0.len().min(rhs.0.len()) {
                            self.0[i].[< $fun_name _assign >](rhs.0[i]);
                        }
                    }
                }
            )*
        }
    }
}

// ----------- TArray START -----------
#[derive(Copy, Clone)]
pub struct TArray<T: Copy + Clone, const N: usize>([T; N]);

impl<T: Copy + Clone, const N: usize> TList<[T; N]> for TArray<T, N> {
    fn get(&self) -> &[T; N] {
        &self.0
    }

    fn get_mut(&mut self) -> &mut [T; N] {
        &mut self.0
    }

    fn set(&mut self, val: [T; N]) {
        self.0 = val;
    }
}

impl<T: Copy, const N: usize> Index<usize> for TArray<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Copy, const N: usize> IndexMut<usize> for TArray<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Neg<Output = T> + Copy, const N: usize> Neg for TArray<T, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut res: TArray<T, N> = self;

        for val in res.0.iter_mut() {
            *val = val.neg();
        }

        res
    }
}

impl<T: Not<Output = T> + Copy, const N: usize> Not for TArray<T, N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut res: TArray<T, N> = self;

        for val in res.0.iter_mut() {
            *val = val.not();
        }

        res
    }
}

impl_op!(
    array;
    Add, add; Sub, sub; Mul, mul; Div, div; Rem, rem;
    Shl, shl; Shr, shr;
    BitAnd, bitand; BitOr, bitor; BitXor, bitxor
);
// ----------- TArray END -----------

// ----------- TVector START -----------
#[derive(Clone)]
pub struct TVector<T: Copy + Clone>(Vec<T>);

impl<T: Copy + Clone> TList<Vec<T>> for TVector<T> {
    fn get(&self) -> &Vec<T> {
        &self.0
    }

    fn get_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }

    fn set(&mut self, val: Vec<T>) {
        self.0 = val;
    }
}

impl<T: Copy> Index<usize> for TVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Copy> IndexMut<usize> for TVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Neg<Output = T> + Copy> Neg for TVector<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut res: TVector<T> = self;

        for val in res.0.iter_mut() {
            *val = val.neg();
        }

        res
    }
}

impl<T: Not<Output = T> + Copy> Not for TVector<T> {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut res: TVector<T> = self;

        for val in res.0.iter_mut() {
            *val = val.not();
        }

        res
    }
}

impl_op!(
    vector;
    Add, add; Sub, sub; Mul, mul; Div, div; Rem, rem;
    Shl, shl; Shr, shr;
    BitAnd, bitand; BitOr, bitor; BitXor, bitxor
);
// ----------- TVector START -----------

// ----------- All START -----------
impl<T: Copy, const N: usize> From<[T; N]> for TArray<T, N> {
    fn from(rhs: [T; N]) -> Self {
        TArray(rhs)
    }
}

impl<T: Copy> From<Vec<T>> for TVector<T> {
    fn from(rhs: Vec<T>) -> Self {
        TVector(rhs)
    }
}

impl<T: Copy, const N: usize> From<TArray<T, N>> for TVector<T> {
    fn from(rhs: TArray<T, N>) -> Self {
        TVector(rhs.0.to_vec())
    }
}
// ----------- All END -----------
