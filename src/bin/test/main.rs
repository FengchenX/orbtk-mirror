mod hello_grpc;

#[macro_use]
extern crate simple_excel_writer as excel;
use excel::*;

use calamine::{Reader, Xlsx, open_workbook};

use hello_grpc::HelloService;
use hello_grpc::*;
use grpc::{ServerHandlerContext, ServerResponseUnarySink, ServerRequestSingle, ClientStubExt};

fn main() {

    println!("Rust gRPC client demo.");
    let client = HelloServiceClient::new_plain("0.0.0.0",30303,Default::default()).unwrap();
    let mut req =  Hello



    let mut wb = Workbook::create("./src/bin/test/b.xlsx");
    let mut sheet = wb.create_sheet("Sheet1");
    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["Name", "Title","Success","Remark"])?;
        sw.append_row(row!["Amy", "Manager", true])
    }).expect("write excel error!");

    wb.close().expect("close excel error!");

    let mut i=0;
    while i<1000 {
       i+=1;
        let mut my_excel: Xlsx<_> = open_workbook("./src/bin/test/a.xlsx").unwrap();
        if let Some(Ok(r)) = my_excel.worksheet_range("Sheet1") {
            for row in r.rows() {
                println!("row={:?}, row[0]={:?}", row, row[0]);
            }
        }
    }
}