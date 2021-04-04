#![allow(dead_code)]
#![allow(warnings)]
use report_tool::ds::{DataSet, DataSetConfig};
use report_tool::writer;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use xlsxwriter::Workbook;

const DATA_PATH: &'static str = "./";
fn main() {
    let time = std::time::Instant::now();
    let path_meta = "/home/faisal/meta2.json";
    let path_input = "/home/faisal/input2.csv";
    let output_path = "/home/faisal/output.xlsx";
    let lng = "ja".to_string();
    let created_year = 2020 as u16;

    match writer::create_output_file(&path_meta, &path_input, output_path, &lng, created_year)
    {
        Ok(res) => println!("{:?}", res),
        Err(e) => {
            // println!("{:?}", e);
            println!("{}", e.to_string());
        }
    }

    let time = time.elapsed().as_millis();
    println!("Finished processing in {} milliseconds", time);

}
