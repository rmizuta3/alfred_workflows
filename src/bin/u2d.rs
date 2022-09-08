use chrono::{DateTime, Local, TimeZone};
use std::io;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let unixtime = args[0].parse::<i64>().unwrap();
    let dt: DateTime<Local> = Local.timestamp(unixtime, 0);

    let _ = alfred::json::write_items(
        io::stdout(),
        &[alfred::ItemBuilder::new("U2D")
            .arg(dt.to_string())
            .subtitle("convert unixtime to datetime")
            //.icon_filetype("public.folder")
            .into_item()],
    );
}
