pub trait Increment {
    type Max;
    fn over(&self, max: &Self::Max) -> bool;
    fn inc(&mut self, max: &Self::Max);
}

impl Increment for usize {
    type Max = usize;

    fn over(&self, max: &Self::Max) -> bool {
        (self + 1) >= *max
    }

    fn inc(&mut self, max: &Self::Max) {
        if !self.over(max) {
            *self += 1;
        } else {
            *self = 0;
        }
    }
}

impl<A, B, M> Increment for (A, B)
where
    A: Increment<Max = M>,
    B: Increment<Max = M>,
{
    type Max = M;

    fn over(&self, max: &Self::Max) -> bool {
        self.0.over(max)
    }

    fn inc(&mut self, max: &Self::Max) {
        self.1.inc(max);
        if self.1.over(max) {
            self.0.inc(max);
        }
    }
}

pub struct Combinator<T, I>
where
    I: Increment,
{
    data: Vec<T>,
    indexs: I,
}

impl<'a, T: Copy> Iterator for Combinator<T, (usize, usize)> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let (ia, ib) = self.indexs;
        if let (Some(a), Some(b)) = (self.data.get(ia), self.data.get(ib)) {
            self.indexs.inc(&self.data.len());
            Some((*a, *b))
        } else {
            None
        }
    }
}

impl<'a, T: Copy> Iterator for Combinator<T, ((usize, usize), usize)> {
    type Item = (T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let ((ia, ib), ic) = self.indexs;
        if let (Some(a), Some(b), Some(c)) =
            (self.data.get(ia), self.data.get(ib), self.data.get(ic))
        {
            self.indexs.inc(&self.data.len());
            Some((*a, *b, *c))
        } else {
            None
        }
    }
}

pub fn pairs<T>(data: Vec<T>) -> Combinator<T, (usize, usize)> {
    Combinator {
        data,
        indexs: (0, 0),
    }
}

pub fn trits<T>(data: Vec<T>) -> Combinator<T, ((usize, usize), usize)> {
    Combinator {
        data,
        indexs: ((0, 0), 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_number() {
        let mut val = 32;
        val.inc(&33);
        assert_eq!(val, 33);
        val.inc(&33);
        assert_eq!(val, 0);
    }

    #[test]
    fn increment_pair() {
        let mut val = (32, 32);
        val.inc(&33);
        assert_eq!(val, (32, 33));
        val.inc(&33);
        assert_eq!(val, (33, 0));
        val.inc(&33);
        assert_eq!(val, (33, 1));
        val = (33, 33);
        val.inc(&33);
        assert_eq!(val, (0, 0));
    }

    #[test]
    fn increment_trite() {
        let mut val = ((32, 32), 32);
        val.inc(&33);
        assert_eq!(val, ((32, 32), 33));
        val.inc(&33);
        assert_eq!(val, ((32, 33), 0));
    }
}
