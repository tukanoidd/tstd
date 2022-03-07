pub enum Either<S, T> {
    Left(S),
    Right(T),
}

impl<S, T> Either<S, T> {
    pub fn is_left(&self) -> bool {
        match self {
            Either::Left(_) => true,
            Either::Right(_) => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            Either::Left(_) => false,
            Either::Right(_) => true,
        }
    }

    pub fn left(&self) -> Option<&S> {
        match self {
            Either::Left(s) => Some(s),
            Either::Right(_) => None,
        }
    }

    pub fn left_mut(&mut self) -> Option<&mut S> {
        match self {
            Either::Left(s) => Some(s),
            Either::Right(_) => None,
        }
    }

    pub fn right(&self) -> Option<&T> {
        match self {
            Either::Left(_) => None,
            Either::Right(t) => Some(t),
        }
    }

    pub fn right_mut(&mut self) -> Option<&mut T> {
        match self {
            Either::Left(_) => None,
            Either::Right(t) => Some(t),
        }
    }
}
