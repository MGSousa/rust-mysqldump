use cli_table::{print_stdout, Cell, Color, Style, Table};
use flate2::Compression;
use gzp::{deflate::Gzip, ZBuilder};
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

pub fn compress_file(file_path: &str, gzip_path: &str, level: &str) -> std::io::Result<()> {
    let data = std::fs::read(file_path)?;

    let file = std::fs::File::create(gzip_path)?;

    let level_int = match level {
        "BEST" => Compression::best(),
        "FASTEST" | "FAST" => Compression::fast(),
        // as a default
        _ => Compression::default(),
    };

    let mut z = ZBuilder::<Gzip, _>::new()
        .compression_level(level_int)
        .from_writer(file);
    z.write_all(&data)?;

    Ok(())
}
