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
    let path_meta = "/home/faisal/meta_hot.json";
    let path_input = "/home/faisal/input_hot.csv";
    let output_path = "/home/faisal/output_hot.xlsx";
    let lng = "ja".to_string();
    let created_year = 2021 as u16;
    let ignores = vec![("purchase-status".to_string(), "評価済み".to_string())];

    match writer::create_output_file(&path_meta, &path_input, output_path, &lng, created_year, ignores)
    {
        Ok(res) => println!("{:?}", res),
        Err(e) => {
            // println!("{:?}", e);
            println!("{}", e.to_string());
        }
    }
    let config = report_tool::ds::config::provide_ignore_criteria_constants();
    // print!("{:#?}", config);


    let time = time.elapsed().as_millis();
    println!("Finished processing in {} milliseconds", time);

}
