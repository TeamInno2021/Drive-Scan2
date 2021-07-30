use dslib::{__init, query, scan};

fn main() {
    __init();

    scan("C:\\".into()).unwrap();
    println!("{:#?}", query("C:\\".into()).unwrap());
}
