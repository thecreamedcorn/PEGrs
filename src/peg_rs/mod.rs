mod grammars;
pub mod input;

#[test]
fn the_test() {
    let mut x: i64 = 5;
    let y = &mut x;
    {
        let test: &mut i64 = y;
        *test = 6;
    }
    println!("{:?}", *y);
}