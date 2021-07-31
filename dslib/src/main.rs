use dslib::{__init, query, scan};

fn main() {
    __init();

    // scan("W:\\Coding\\Drive-Scan2\\dslib\\src".into()).unwrap();
    scan("W:\\Coding\\Drive-Scan2\\dslib\\src\\windows".into()).unwrap();
    println!("{:#?}", query("W:\\".into()).unwrap());
}
