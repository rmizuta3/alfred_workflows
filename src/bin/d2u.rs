use chrono::{DateTime, Local, TimeZone};
use std::io;

fn main() {
    let input_args: Vec<String> = std::env::args().skip(1).collect();

    let input_args_str: String = input_args.get(0).unwrap().to_string();
    let args: Vec<&str> = input_args_str.split_whitespace().collect();

    let yyyymmdd = args[0];
    let mut hms = "";
    if args.len() == 1 {
        //日時を指定しないケース
        hms = "00:00:00";
    } else {
        hms = args.get(1).unwrap();
    }

    let dt: DateTime<Local> = Local
        .datetime_from_str(&(yyyymmdd.to_string() + hms), "%Y%m%d %H:%M:%S")
        .unwrap();
    let timestamp: i64 = dt.timestamp();

    let _ = alfred::json::write_items(
        io::stdout(),
        &[alfred::ItemBuilder::new(timestamp.to_string())
            .arg(timestamp.to_string())
            .subtitle("convert datetime to unixtime")
            //.icon_filetype("public.folder")
            .into_item()],
    );
}
