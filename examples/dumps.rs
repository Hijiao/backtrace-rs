extern crate backtrace;

use backtrace::Backtrace;

fn main() {
    let mut buf = Vec::with_capacity(10);
    let dumps_result = Backtrace::dumps_emergency(&mut buf);

    if dumps_result.is_err() {
        println!("dumps error: {:?}", dumps_result);
    } else {
        println!("{}", String::from_utf8(buf).unwrap());
    }
}
