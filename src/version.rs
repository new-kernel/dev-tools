pub(crate) const VERSION: (i32, i32, i32) = (1, 0, 0);

pub(crate) fn print_version() {
    println!("Novusk Dev Tools version: {}.{}.{}", VERSION.0, VERSION.1, VERSION.2);
}
