use bytes_writable::BytesWritable;
use serde::Serialize;
use the_macro::{log_args, repeat, BytesWritable};

mod bytes_writable;

#[log_args(logger = println)]
fn print_something_20_times(something: &str, n: usize, x: Vec<u8>) {
    repeat! { 20 => {
        println!("{}", something);
    }};
}

#[test]
fn test_expr_macro() {
    print_something_20_times("Hello, World!", 1, vec![3, 2, 1]);

    let _ = {
        let mut x = 1; // Stmt::Local

        x += 1; // Stmt::Item

        x // Expr
    };

    panic!();
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, BytesWritable)]
struct MySpecialStruct<T: std::fmt::Debug, U>
where
    T: PartialEq,
{
    identifier: [u8; 4],
    some_value: T,
    items: Vec<U>,
}

#[test]
fn testing_bytes_writable() {
    let my_special_struct = MySpecialStruct::<u32, u32> {
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
