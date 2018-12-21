fn main() {
    let mut d : i64  = 65536;
    let mut e : i64  = 14464005;
    let mut max_e : i64  = 0;
    loop {
        e = e + (d & 255);
        e = e & 16777215;
        e = e * 65899;
        e = e & 16777215;
        d = d / 256;
        println!("e: {}, {}", e, max_e);
        if e > max_e {
            max_e = e;
        } else {
//            break;
        }
    }
}
