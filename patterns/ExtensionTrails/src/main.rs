std::hash::Hash
std::collections::HashSet
#![allow(unused)]

pub trait IteratorExt: Iterator {
    fn unique(self) -> Vec<Self::Item>
    where
        Self::Item: Eq + Hash,
    {
        use HashSet;
        let mut seen = HashSet::new();
        self.filter(|item| seen.insert(item.clone())).collect()
    }
}

pub struct UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash,
{
    seen: HashSet<I::Item>,
    iter: std::iter::Peekable<I>,
}

impl<I> Iterator for UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if self.seen.insert(item.clone()) {
                return Some(item);
            }
        }
        None
    }
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 4, 1, 5];
    let unique_numbers: Vec<_> = numbers.into_iter().unique();
    println!("{:?}", unique_numbers);

    let numbers = vec![1, 2, 2, 3, 4, 1, 5];
    let unique_numbers: Vec<_> = UniqueIterator {
        seen: HashSet::new(),
        iter: numbers.into_iter().peekable(),
    }
    .collect();
    println!("{:?}", unique_numbers);
}
