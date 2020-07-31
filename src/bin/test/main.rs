#[macro_use]
extern crate simple_excel_writer as excel;

use excel::*;

fn main() {
    let mut wb = Workbook::create("./src/bin/test/b.xlsx");
    let mut sheet = wb.create_sheet("Sheet1");
    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["Name", "Title","Success","Remark"])?;
        sw.append_row(row!["Amy", "Manager", true])
    }).expect("write excel error!");

    wb.close().expect("close excel error!");
}