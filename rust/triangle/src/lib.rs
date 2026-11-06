use std::ops::Add;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: Copy + PartialOrd + Add<Output = T>,
{
    pub fn build([a, b, c]: [T; 3]) -> Option<Triangle<T>> {
        if c < a + b && a < b + c && b < c + a {
            Some(Triangle { a, b, c })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.a == self.c || self.b == self.c
    }
}
