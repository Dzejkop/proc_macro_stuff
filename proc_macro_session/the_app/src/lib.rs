use the_macro::{identity, log_args, repeat, BytesWritable};

#[test]
fn repeat_test() {
    let mut n = 0;
    repeat! { 10 => {
        n += 1;
    }}

    assert_eq!(n, 10);
}

#[derive(Debug)]
struct Whatever;

impl Whatever {
    #[log_args]
    fn print_something_20_times(&self, something: &str, n: usize, x: Vec<u8>) {
        println!("&self = {:?}", &self);
        repeat! { 20 => {
            println!("{}", something);
        }};
    }
}

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

#[derive(BytesWritable)]
struct MySpecialStruct {
    identifier: [u8; 4],
    some_value: u32,
    items: Vec<u32>,
}

#[test]
fn testing_bytes_writable() {
    let my_special_struct = MySpecialStruct {
        identifier: [1, 2, 3, 4],
        some_value: 321,
        items: vec![4, 3, 2, 1],
    };

    let mut buf: Vec<u8> = Vec::new();
    my_special_struct.write_bytes(&mut buf).unwrap();

    assert_eq!(
        vec![
            1, 2, 3, 4, // raw identifier
            0, 0, 1, 65, // 321 as be bytes
            0, 0, 0, 0, 0, 0, 0, 4, // items.len() as be bytes
            0, 0, 0, 4, // 4u32 as be bytes
            0, 0, 0, 3, // 3u32 as be bytes
            0, 0, 0, 2, // 2u32 as be bytes
            0, 0, 0, 1, // 1u32 as be bytes
        ],
        buf
    );
}
