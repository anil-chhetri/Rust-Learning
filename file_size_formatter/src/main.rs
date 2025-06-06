use std::env;

#[derive(Debug)]
enum SizeKind {
    B(f64),
    Kb(f64),
    Mb(f64),
    Gb(f64),
    Error(String),
}

#[derive(Debug)]
struct Size {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl SizeKind {
    fn get_bytes(size_kind: &SizeKind) -> Result<String, String> {
        match size_kind {
            SizeKind::B(b) => Ok(format!("{} Bytes", b)),
            SizeKind::Kb(kb) => Ok(format!("{} Bytes", (kb * 1_000.0))),
            SizeKind::Mb(mb) => Ok(format!("{} Bytes", (mb * 1_000_000.0))),
            SizeKind::Gb(gb) => Ok(format!("{} Bytes", (gb * 1_000_000_000.0))),
            SizeKind::Error(_) => Err("cannot convert ".to_string()),
        }
    }

    fn get_kilo_bytes(size_kind: &SizeKind) -> Result<String, String> {
        match size_kind {
            SizeKind::B(b) => Ok(format!("{} Kilobytes", b / 1_000.0)),
            SizeKind::Kb(kb) => Ok(format!("{} Kilobytes", (kb))),
            SizeKind::Mb(mb) => Ok(format!("{} Kilobytes", (mb * 1_000.0))),
            SizeKind::Gb(gb) => Ok(format!("{} Kilobytes", (gb * 1_000_000_000.0))),
            SizeKind::Error(_s) => Err("cannot convert ".to_string()),
        }
    }

    fn get_mega_bytes(size_kind: &SizeKind) -> Result<String, String> {
        match size_kind {
            SizeKind::B(b) => Ok(format!("{} Megabytes", b / 1_000_000.0)),
            SizeKind::Kb(kb) => Ok(format!("{} Megabytes", (kb / 1_000.0))),
            SizeKind::Mb(mb) => Ok(format!("{} Megabytes", (mb))),
            SizeKind::Gb(gb) => Ok(format!("{} Megabytes", (gb * 1_000.0))),
            SizeKind::Error(_s) => Err("cannot convert ".to_string()),
        }
    }

    fn get_giga_bytes(size_kind: &SizeKind) -> Result<String, String> {
        match size_kind {
            SizeKind::B(b) => Ok(format!("{} Gigabytes", b / 1_000_000_000.0)),
            SizeKind::Kb(kb) => Ok(format!("{} Gigabytes", (kb / 1_000_000.0))),
            SizeKind::Mb(mb) => Ok(format!("{} Gigabytes", (mb / 1_000.0))),
            SizeKind::Gb(gb) => Ok(format!("{} Gigabytes", (gb))),
            SizeKind::Error(_s) => Err("cannot convert ".to_string()),
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let paramters: Vec<&str> = (&args[1]).split(" ").collect();

    println!("first paramater value: {}", paramters[0]);
    println!("first paramater value: {}", paramters[1]);

    let number_part: f64 = paramters[0].parse().unwrap_or(0.0);

    let size_kind = match paramters[1] {
        "Kb" => SizeKind::Kb(number_part),
        "B" => SizeKind::B(number_part),
        "Mb" => SizeKind::Mb(number_part),
        "Gb" => SizeKind::Gb(number_part),
        _ => SizeKind::Error(format!("Invalid size part: {}", paramters[1])),
    };

    println!("sizeKind: {:#?}", size_kind);

    let bytes = SizeKind::get_bytes(&size_kind);
    let kilo_bytes = SizeKind::get_kilo_bytes(&size_kind);
    let mega_bytes = SizeKind::get_mega_bytes(&size_kind);
    let giga_bytes = SizeKind::get_giga_bytes(&size_kind);

    let all_size = Size {
        bytes: bytes.unwrap_or("can't convert".to_string()),
        kilobytes: kilo_bytes.unwrap_or("can't convert".to_string()),
        megabytes: mega_bytes.unwrap_or("can't convert".to_string()),
        gigabytes: giga_bytes.unwrap_or("can't convert".to_string()),
    };

    println!(
        "bytes value of {:?} is {}",
        paramters,
        SizeKind::get_bytes(&size_kind).unwrap_or("can't convert".to_string())
    );

    println!("Displaying all sizes \n {:#?}", all_size);
}
