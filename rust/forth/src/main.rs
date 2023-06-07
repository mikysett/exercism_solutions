use forth::Forth;

#[allow(unused)]
fn main() {
    let mut f = Forth::new();
    dbg!(f.eval(": dup-twice dup dup ;"));
    dbg!(f.eval("1 dup-twice"));
}
