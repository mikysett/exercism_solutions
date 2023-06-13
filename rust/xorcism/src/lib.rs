use std::{borrow::Borrow, iter::Cycle, slice::Iter};

#[derive(Clone)]
pub struct Xorcism<'a> {
    key: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        Self {
            key: key.as_ref().iter().cycle(),
        }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut().for_each(|byte| *byte = self.xor(*byte));
    }

    pub fn munge<'b, Data: 'b>(
        &'b mut self,
        data: Data,
    ) -> impl Iterator<Item = u8> + Captures<'a> + 'b
    where
        Data: IntoIterator,
        Data::IntoIter: 'b,
        Data::Item: Borrow<u8>,
    {
        data.into_iter().map(|byte| self.xor(*byte.borrow()))
    }

    fn xor(&mut self, byte: u8) -> u8 {
        *self.key.next().unwrap() ^ byte
    }
}

pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}
