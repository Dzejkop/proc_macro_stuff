use std::io::Write;

pub trait BytesWritable {
    fn write_bytes<W>(&self, write_target: W) -> std::io::Result<()>
    where
        W: Write;
}

impl BytesWritable for u8 {
    fn write_bytes<W>(&self, mut write_target: W) -> std::io::Result<()>
    where
        W: Write,
    {
        write_target.write_all(&[*self])
    }
}

impl<'a, T> BytesWritable for &'a [T]
where
    T: BytesWritable,
{
    fn write_bytes<W>(&self, mut write_target: W) -> std::io::Result<()>
    where
        W: Write,
    {
        self.len().write_bytes(&mut write_target)?;

        for item in self.iter() {
            item.write_bytes(&mut write_target)?;
        }

        Ok(())
    }
}

impl BytesWritable for usize {
    fn write_bytes<W>(&self, mut write_target: W) -> std::io::Result<()>
    where
        W: Write,
    {
        write_target.write_all(&self.to_be_bytes())
    }
}

impl BytesWritable for u32 {
    fn write_bytes<W>(&self, mut write_target: W) -> std::io::Result<()>
    where
        W: Write,
    {
        write_target.write_all(&self.to_be_bytes())
    }
}

impl<T> BytesWritable for Vec<T>
where
    T: BytesWritable,
{
    fn write_bytes<W>(&self, write_target: W) -> std::io::Result<()>
    where
        W: Write,
    {
        self.as_slice().write_bytes(write_target)
    }
}

impl<T, const N: usize> BytesWritable for [T; N]
where
    T: BytesWritable,
{
    fn write_bytes<W>(&self, mut write_target: W) -> std::io::Result<()>
    where
        W: Write,
    {
        for item in self.iter() {
            item.write_bytes(&mut write_target)?;
        }

        Ok(())
    }
}