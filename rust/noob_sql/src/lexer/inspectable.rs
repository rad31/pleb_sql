use super::tokens::Token;

pub trait Inspectable {
    fn starts<T: Token>(&self) -> bool;
    fn terminates<T: Token>(&self, prev: Option<char>) -> bool;
}

impl Inspectable for char {
    fn starts<T: Token>(&self) -> bool {
        T::is_start(*self)
    }

    fn terminates<T: Token>(&self, prev: Option<char>) -> bool {
        T::is_end(*self, prev)
    }
}

impl Inspectable for &char {
    fn starts<T: Token>(&self) -> bool {
        T::is_start(**self)
    }

    fn terminates<T: Token>(&self, prev: Option<char>) -> bool {
        T::is_end(**self, prev)
    }
}
