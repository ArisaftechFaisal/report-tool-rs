#![allow(dead_code)]
#![allow(warnings)]
use report_tool::ds::{DataSet, DataSetConfig};
use report_tool::writer;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use xlsxwriter::Workbook;
// use time_graph;

const DATA_PATH: &'static str = "./";
fn main() {
    // time_graph::enable_data_collection(true);
    let time = std::time::Instant::now();
    let path_meta = Path::new(DATA_PATH).join("meta.json");
    let path_data = Path::new(DATA_PATH).join("input.csv");
    // let path_output = Path::new(DATA_PATH).join("output.xlsx");
    let output_name = "output.xlsx";

    writer::create_output_file(&path_meta, &path_data, output_name).unwrap();
    let time = time.elapsed().as_millis();
    println!("Finished processing in {} milliseconds", time);
    // let graph = time_graph::get_full_graph();
    // let mut graph_file = std::fs::File::create("graph.dot").unwrap();
    // graph_file.write_all(graph.as_dot().as_bytes()).unwrap();

}
