use cli_table::{print_stdout, Cell, Color, Style, Table};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

pub fn print_databases(databases: &Vec<(usize, String, u64)>) {
    let table_rows: Vec<_> = databases
        .into_iter()
        .map(|(i, db, duration)| {
            vec![
                (i + 1)
                    .to_string()
                    .cell()
                    .foreground_color(Some(Color::Yellow)),
                db.cell().foreground_color(Some(Color::Yellow)),
                duration
                    .to_string()
                    .cell()
                    .foreground_color(Some(Color::Yellow)),
            ]
        })
        .collect();

    let table = table_rows.table().title(vec![
        "Index".cell().bold(true),
        "Database Name".cell().bold(true),
        "Execution Time (s)".cell().bold(true),
    ]);

    assert!(print_stdout(table).is_ok());
}

pub fn compress_file(file_path: &str, gzip_path: &str) -> std::io::Result<()> {
    let data = std::fs::read(file_path)?;

    let file = std::fs::File::create(gzip_path)?;
    let mut e = GzEncoder::new(file, Compression::default());
    e.write_all(&data)?;

    Ok(())
}
