use dslib::{__init, query, scan};

fn main() {
    __init();

    scan("/".into()).unwrap();
    println!(
        "{:#?}",
        query("/".into()).unwrap()
    );
}
