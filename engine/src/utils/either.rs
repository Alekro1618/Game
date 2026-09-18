use std::any::Any;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Either<A, B> {
    pub fn left(self) -> A {
        match self {
            Self::Left(a) => a,
            Self::Right(_) => panic!("Value was right. Unwraped as left."),
        }
    }

    pub fn right(self) -> B {
        match self {
            Self::Left(_) => panic!("Value was left. Unwraped as right."),
            Self::Right(b) => b,
        }
    }

    pub fn swap(self) -> Either<B, A> {
        match self {
            Either::Left(a) => Either::Right(a),
            Either::Right(b) => Either::Left(b),
        }
    }

    pub fn is_left(&self) -> bool {
        match self {
            Self::Left(_) => true,
            Self::Right(_) => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            Self::Left(_) => false,
            Self::Right(_) => true,
        }
    }

    pub fn map_left(self, map: impl FnOnce(A) -> A) -> Self {
        match self {
            Self::Left(a) => Self::Left(map(a)),
            other => other,
        }
    }

    pub fn map_right(self, map: impl FnOnce(B) -> B) -> Self {
        match self {
            Self::Right(b) => Either::Right(map(b)),
            other => other,
        }
    }

    pub fn pair(self) -> (Option<A>, Option<B>) {
        match self {
            Self::Left(a) => (Some(a), None),
            Self::Right(b) => (None, Some(b)),
        }
    }

    pub fn expect_left<E>(self, err: E) -> Result<A, E> {
        match self {
            Self::Left(a) => Ok(a),
            Self::Right(_) => Err(err),
        }
    }

    pub fn expect_right<E>(self, err: E) -> Result<B, E> {
        match self {
            Self::Left(_) => Err(err),
            Self::Right(b) => Ok(b),
        }
    }

    pub fn as_ref(&self) -> Either<&A, &B> {
        match self {
            Either::Left(left) => Either::Left(left),
            Either::Right(right) => Either::Right(right),
        }
    }

    pub fn as_left(&self) -> Option<&A> {
        match self {
            Either::Left(left) => Some(left),
            Either::Right(_) => None,
        }
    }

    pub fn as_right(&self) -> Option<&B> {
        match self {
            Either::Left(_) => None,
            Either::Right(right) => Some(right),
        }
    }
}

impl<A : 'static, B : 'static> Either<A, B> {
    pub fn matches<T : 'static>(&self) -> bool {
        let l = self.type_id();
        let r = match self {
            Either::Left(a) => a.type_id(),
            Either::Right(b) => b.type_id(),
        };
        
        l == r
    }
}

impl<T> Either<T, Vec<T>> {
    pub fn single(self) -> T {
        self.left()
    }

    pub fn vec(self) -> Vec<T> {
        self.right()
    }

    pub fn iter(self) -> impl Iterator<Item = T> {
        Into::<Vec<T>>::into(self).into_iter()
    }
}

impl<T> Into<Either<T, Vec<T>>> for Vec<T> {
    fn into(mut self) -> Either<T, Vec<T>> {
        if self.len() == 1 {
            Either::Left(self.pop().unwrap())
        } else {
            Either::Right(self)
        }
    }
}

impl<T> Into<Vec<T>> for Either<T, Vec<T>> {
    fn into(self) -> Vec<T> {
        match self {
            Self::Left(single) => vec![single],
            Self::Right(vec) => vec,
        }
    }
}

impl<A> Into<Either<A, ()>> for Option<A> {
    fn into(self) -> Either<A, ()> {
        match self {
            Some(a) => Either::Left(a),
            None => Either::Right(()),
        }
    }
}

impl<A, B> Either<A, Option<B>> {
    pub fn right_option(self) -> Option<Either<A, B>> {
        match self {
            Self::Left(a) => Some(Either::Left(a)),
            Self::Right(Some(b)) => Some(Either::Right(b)),
            Self::Right(None) => None,
        }
    }
}

impl<A, B> Either<Option<A>, B> {
    pub fn left_option(self) -> Option<Either<A, B>> {
        match self {
            Self::Left(Some(a)) => Some(Either::Left(a)),
            Self::Left(None) => None,
            Self::Right(b) => Some(Either::Right(b)),
        }
    }
}

impl<A, B> Either<Option<A>, Option<B>> {
    pub fn option(self) -> Option<Either<A, B>> {
        match self {
            Self::Left(Some(a)) => Some(Either::Left(a)),
            Self::Right(Some(b)) => Some(Either::Right(b)),
            _ => None,
        }
    }
}



#[macro_export]
macro_rules! impl_either {
    (
        $left:ty => $left_fn:ident,
        $right:ty => $right_fn:ident,
    ) => {
        impl crate::utils::Either<$left, $right> {
            pub fn $left_fn(self) -> $left {
                self.left()
            }
            
            pub fn $right_fn(self) -> $right {
                self.right()
            }
        }
    };

    (
        $left:ty => $left_fn:ident,
        $right:ty => $right_fn:ident,
        $( $param:ident ),*
    ) => {
        impl<$($param),*> crate::utils::Either<$left, $right> {
            pub fn $left_fn(self) -> $left {
                self.left()
            }
            
            pub fn $right_fn(self) -> $right {
                self.right()
            }
        }
    };

    (
        $left:ty => $left_fn:ident,
        $right:ty => $right_fn:ident,
        generics: [$( $param:ident ),*],
        bounds: $($bound:tt)*
    ) => {
        impl<$( $param ),*> crate::utils::Either<$left, $right>
        where
            $($bound)* {
            pub fn $left_fn(self) -> $left {
                self.left()
            }
            
            pub fn $right_fn(self) -> $right {
                self.right()
            }
        }
    };
}