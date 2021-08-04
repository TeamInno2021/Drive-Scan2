use dslib::{__init, query, scan};

fn main() {
    __init();

    scan("/home/benjamin/".into()).unwrap();
    println!("{:#?}", query("C:\\".into()).unwrap());
}
