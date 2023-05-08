use clap::{arg,Command};
fn main() {
    let app = Command::new("Cols")
    .version("1.0")
    .author("hfh1999 <2350827470@qq.com>")
    .about("Choose Text by cols.")
    .arg(arg!( --title <ROWNUM>))
    .arg(arg!(<COLNUM>));

    let matches = app.get_matches();
    let cols_num = matches.get_one::<u8>("COLNUM").unwrap();
}
