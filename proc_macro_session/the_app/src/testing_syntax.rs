fn print_something_20_times(something: &str, n: usize, x: Vec<u8>) {
    println!("#ident = {:?}", something);
    println!("#ident = {:?}", n);
    println!("#ident = {:?}", x);
    {
        repeat! { 20 => { println! ("{}", something) ; } };
    }
}
