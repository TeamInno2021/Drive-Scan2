use dslib::_scan;

fn main() {
    let res = _scan("C:\\".into()).unwrap();
    println!("{:#?}", res);
}
