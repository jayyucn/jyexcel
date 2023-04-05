use calamine::{open_workbook, Error, Reader, Xlsx};

pub fn read_excels() -> Result<(), Error> {
    let path = format!("{}/test.xlsx", env!("CARGO_MANIFEST_DIR"));
    read_excel_by_path(&path)
}

pub fn read_excel_by_path(path: &str) -> Result<(), Error> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let sheet_name = workbook.sheet_names()[0].clone();
    let range = workbook
        .worksheet_range(&sheet_name)
        .ok_or("Cannot find range")??;

    for (index, row) in range.rows().enumerate() {
        if index == 0 {
            println!("type line");
        }
        for cell in row.iter() {
            match cell {
                calamine::DataType::Float(num) => println!("num-{}", num),
                // calamine::DataType::String(text) => println!("str-{}", text),
                _ => {
                    // println!("unexpected-{}", cell.to_string());
                }
            }
        }
    }
    Ok(())
}
