use std::time::SystemTime;
use time::OffsetDateTime;

fn main() {
    let t: OffsetDateTime = SystemTime::now().into();
    let bt = format!(
        "{:0>4}{:0>2}{:0>2}{:0>2}{:0>2}",
        t.year(),
        t.month() as u8,
        t.day(),
        t.hour(),
        t.minute()
    );

    println!("cargo:rustc-env=BUILD_TIME={}", bt);
}
