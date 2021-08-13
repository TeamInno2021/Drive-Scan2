use dslib::{__init, query, scan};

fn main() {
    __init();

    scan("r/home".into()).unwrap();
    println!("{:#?}", query(r"/home/benjamin/Documents/".into()).unwrap());
}
