use calamine::{Reader, Xlsx, open_workbook};

fn main() {
    let mut my_excel: Xlsx<_> = open_workbook("./src/bin/test/a.xlsx").unwrap();
    if let Some(Ok(r)) = my_excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    }
}