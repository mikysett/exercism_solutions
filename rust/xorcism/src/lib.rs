use std::{borrow::Borrow, u8};

#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    key_pos: usize,
    key_len: usize,
}

impl<'a> Xorcism<'a> {
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        Self {
            key: key.as_ref(),
            key_pos: 0,
            key_len: key.as_ref().len(),
        }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut().for_each(|byte| *byte = self.xor(*byte));
    }

    pub fn munge<'b, Data: 'b>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + 'b
    where
        'a: 'b,
        Data: IntoIterator,
        <Data as IntoIterator>::IntoIter: 'b,
        Data::Item: Borrow<u8>,
    {
        // data.into_iter().map(|byte| self.xor(*byte.borrow()))

        data.into_iter().map(|byte| {
            let key_byte = self.key[self.key_pos];
            let res = key_byte ^ *byte.borrow();
            self.key_pos = (self.key_pos + 1) % self.key_len;

            res
        })
    }

    fn xor(&mut self, byte: u8) -> u8 {
        let key_byte = self.key[self.key_pos];
        let res = key_byte ^ byte;
        self.key_pos = (self.key_pos + 1) % self.key_len;

        res
    }
}
