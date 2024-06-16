// contains Stream and BufferedStream class and interop with lists and generators
use std::vec::Vec;

enum StreamNodeType {
    YIELD
}

struct Stream<T: Sized + Default + Copy> {
    base: Vec<T>,
    cur: usize
}

impl <T: Sized + Default + Copy> Stream<T> {
    pub fn from_list(list: Vec<T>) -> Self {
        Self { base: list }
    }

    pub fn from_generator<const LEN: usize>(initial: T, next: fn(T) -> T) -> Self {
        let mut base: Vec<T> = Vec::new();
        let mut cur = initial;
        base.push(initial);

        for _ in 1..LEN {
            cur = next(cur);
            base.push(cur);
        }

        Self { base }
    }
}
