use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    input: R,
    count_reads: usize,
    size_reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            input: wrapped,
            count_reads: 0,
            size_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.input
    }

    pub fn bytes_through(&self) -> usize {
        self.size_reads
    }

    pub fn reads(&self) -> usize {
        self.count_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.input.read(buf).map(|size| {
            self.count_reads += 1;
            self.size_reads += size;
            size
        })
    }
}

pub struct WriteStats<W> {
    output: W,
    count_writes: usize,
    size_writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            output: wrapped,
            count_writes: 0,
            size_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.output
    }

    pub fn bytes_through(&self) -> usize {
        self.size_writes
    }

    pub fn writes(&self) -> usize {
        self.count_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.output.write(buf).map(|size| {
            self.count_writes += 1;
            self.size_writes += size;
            size
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.output.flush()
    }
}
