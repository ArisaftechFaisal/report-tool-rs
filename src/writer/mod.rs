use crate::ds::{DataSet, DataSetConfig, CrosstabType};
use crate::errors::RustlyzerError;
use chrono::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use xlsxwriter::{FormatColor, Workbook, WorksheetRow, WorksheetCol, Worksheet, Format, FormatAlignment, ChartType, ChartFill};
use crate::ds::table::{Table, TableWithMeta, SpecialCase};

pub fn create_output_file(
    meta_path: &str,
    input_path: &str,
    output_path: &str,
    lng: &str,
    created_year: u16

) -> Result<(), RustlyzerError> {
    let input_path = Path::new(input_path);
    let meta_path = Path::new(meta_path);
    let output_path =  Path::new(output_path);
    let io_read_time = std::time::Instant::now();
    let mut meta = String::new();
    let mut data = String::new();

    File::open(meta_path)?.read_to_string(&mut meta)?;
    File::open(input_path)?.read_to_string(&mut data)?;
    let temp_file_name = format!("{}.xlsx", Utc::now().format("%Y%m%d_%H%M%S%f").to_string());
    let config = DataSetConfig::new(lng.to_string(), created_year)?;

    let dataset = DataSet::from_data(meta.as_ref(), config, data.as_ref())?;
    let io_read_time = io_read_time.elapsed().as_millis();
    println!("IO read time is {} milliseconds", io_read_time);
    println!("Loading done!");
    let workbook = Workbook::new(&temp_file_name);

    // Formats
    let mut bg_normal = workbook.add_format().set_bg_color(FormatColor::Custom(0xc6efce))
        .set_font_color(FormatColor::Custom(0x006100));
    let mut bg_highlight = workbook.add_format().set_bg_color(FormatColor::Custom(0xffeb9c))
        .set_font_color(FormatColor::Custom(0x9c6500));
    let mut bg_highlight_red = workbook.add_format().set_bg_color(FormatColor::Custom(0xffc7ce))
        .set_font_color(FormatColor::Custom(0x9c0006));
    let mut num_format = workbook.add_format().set_align(FormatAlignment::Right);
    let mut perc_format = workbook.add_format().set_num_format(r#"##.00\%"#);

    // FKC Rawdata
    {
        let sheet_name = "fkc_rawdata";
        let table_fkc = dataset.get_fkc_raw_table()?;
        let mut fkc_sheet = workbook.add_worksheet(Some(sheet_name))?;
        write_table(0, 0, table_fkc, &mut fkc_sheet, &bg_normal, &bg_highlight, &num_format, &perc_format);
    }

    // IT Rawdata
    {
        let sheet_name = "it_rawdata";
        let table_it = dataset.get_it_raw_table()?;
        let mut it_sheet = workbook.add_worksheet(Some(sheet_name))?;
        write_table(0, 0, table_it, &mut it_sheet, &bg_normal, &bg_highlight, &num_format, &perc_format);
    }

    // User graphs
    {
        let sheet_name = "user_graph";
        let mut user_graph_sheet = workbook.add_worksheet(Some(sheet_name))?;
        let mut coord = CellCoord::new(0, 0);
        // Age 1060
        {
            let table = dataset.get_user_graph_table_age1060()?;
            coord = write_table(coord.row, coord.col, table,
                                &mut
                user_graph_sheet, &bg_normal,
                                &bg_highlight, &num_format, &perc_format)?;
        }
        coord = CellCoord::new(coord.row + 2, 0);
        // Age 1070
        {
            let table = dataset.get_user_graph_table_age1070()?;
            coord = write_table(coord.row, coord.col, table, &mut
                user_graph_sheet, &bg_normal,
                        &bg_highlight, &num_format, &perc_format)?;
        }
        coord = CellCoord::new(coord.row + 2, 0);
        // Gender
        {
            let table = dataset.get_user_graph_table_gender()?;
            coord = write_table(coord.row, coord.col, table, &mut user_graph_sheet, &bg_normal,
                                &bg_highlight, &num_format, &perc_format)?;
        }
        coord = CellCoord::new(coord.row + 2, 0);
        // Marital
        {
            let table = dataset.get_user_graph_table_marital()?;
            coord = write_table(coord.row, coord.col, table, &mut user_graph_sheet, &bg_normal,
                                &bg_highlight, &num_format, &perc_format)?;
        }
        coord = CellCoord::new(coord.row + 2, 0);
        // Children
        {
            let table = dataset.get_user_graph_table_children()?;
            coord = write_table(coord.row, coord.col, table, &mut user_graph_sheet, &bg_normal,
                                &bg_highlight, &num_format, &perc_format)?;
        }
        coord = CellCoord::new(0, 4);
        // Job
        {
            let table = dataset.get_user_graph_table_job()?;
            coord = write_table(coord.row, coord.col, table, &mut user_graph_sheet, &bg_normal,
                                &bg_highlight, &num_format, &perc_format)?;
        }
        coord = CellCoord::new(coord.row + 3, 4);
        // Region
        {
            let table = dataset.get_user_graph_table_region()?;
            coord = write_table(coord.row, coord.col, table, &mut user_graph_sheet, &bg_normal,
                                &bg_highlight, &num_format, &perc_format)?;
        }
        coord = CellCoord::new(coord.row + 3, 4);
        // Income
        {
            let table = dataset.get_user_graph_table_income()?;
            coord = write_table(coord.row, coord.col, table, &mut user_graph_sheet, &bg_normal,
                                &bg_highlight, &num_format, &perc_format)?;
        }

        // Chart
        {
            let mut chart_fill_1 = ChartFill::new();
            chart_fill_1.color = FormatColor::Custom(0xccffff);
            let mut chart_fill_2 = ChartFill::new();
            chart_fill_2.color = FormatColor::Custom(0xffe699);
            let mut chart_fill_3 = ChartFill::new();
            chart_fill_3.color = FormatColor::Custom(0xffff99);
            let mut chart_fill_4 = ChartFill::new();
            chart_fill_4.color = FormatColor::Custom(0xffccff);
            let mut chart_fill_5 = ChartFill::new();
            chart_fill_5.color = FormatColor::Custom(0xffcc99);
            let mut chart_fill_6 = ChartFill::new();
            chart_fill_6.color = FormatColor::Custom(0xcc99ff);
            let mut chart_fill_7 = ChartFill::new();
            chart_fill_7.color = FormatColor::Custom(0xd2ff79);
            // == with n-number ==
            let mut starting_col = 9;
            {
               // -- First column --
            // Label
            user_graph_sheet.merge_range(0,starting_col, 0, starting_col + 7,"n数あり \
            未既婚/子供の人数/世帯年収", Some
                (&bg_highlight))?;
            // Marital chart
            {
                let mut chart = workbook.add_chart((ChartType::Pie));
                let mut series = chart.add_series(None, None);
                series.set_name("未既婚");
                // series.set_fill()
                series.set_fill(&chart_fill_1);
                series.set_categories(sheet_name, 25, 0, 26, 0);
                series.set_values(sheet_name, 25, 1, 26, 1);
                user_graph_sheet.insert_chart(1, starting_col, &chart);
            }
            //  Children chart
            {
                let mut chart = workbook.add_chart((ChartType::Pie));
                let mut series = chart.add_series(None, None);
                series.set_name("子供の人数");
                series.set_categories(sheet_name, 30, 0, 34, 0);
                series.set_values(sheet_name, 30, 1, 34, 1);
                user_graph_sheet.insert_chart(16, starting_col, &chart);
            }
            //  Income chart
            {
                let mut chart = workbook.add_chart((ChartType::Bar));
                let mut series = chart.add_series(None, None);
                series.set_name("世帯年収");
                series.set_categories(sheet_name, 29, 6, 42, 6);
                series.set_values(sheet_name, 29, 7, 42, 7);
                user_graph_sheet.insert_chart(31, starting_col, &chart);
            }

            // -- Second column --
            // Label
            user_graph_sheet.merge_range(0,starting_col + 9, 0, starting_col + 16,
                                         "n数あり　年代/性別/職業/地域", Some
                (&bg_highlight))?;
            // AgeGroup1060 chart
            {
                let mut chart = workbook.add_chart((ChartType::Pie));
                let mut series = chart.add_series(None, None);
                series.set_name("年代");
                // series.set_fill()
                series.set_fill(&chart_fill_1);
                series.set_categories(sheet_name, 1, 0, 6, 0);
                series.set_values(sheet_name, 1, 1, 6, 1);
                user_graph_sheet.insert_chart(1, starting_col + 9, &chart);
            }
            //  Gender chart
            {
                let mut chart = workbook.add_chart((ChartType::Pie));
                let mut series = chart.add_series(None, None);
                series.set_name("性別");
                series.set_categories(sheet_name, 20, 0, 21, 0);
                series.set_values(sheet_name, 20, 1, 21, 1);
                user_graph_sheet.insert_chart(16, starting_col + 9, &chart);
            }
            // AgeGroup1070 chart
            {
                let mut chart = workbook.add_chart((ChartType::Pie));
                let mut series = chart.add_series(None, None);
                series.set_name("年代");
                // series.set_fill()
                series.set_fill(&chart_fill_1);
                series.set_categories(sheet_name, 10, 0, 16, 0);
                series.set_values(sheet_name, 10, 1, 16, 1);
                user_graph_sheet.insert_chart(31, starting_col + 9, &chart);
            }
            //  Job chart
            {
                let mut chart = workbook.add_chart((ChartType::Bar));
                let mut series = chart.add_series(None, None);
                series.set_name("職業");
                series.set_categories(sheet_name, 1, 6, 12, 6);
                series.set_values(sheet_name, 1, 7, 12, 7);
                user_graph_sheet.insert_chart(46, starting_col + 9, &chart);
            }
            //  Region chart
            {
                let mut chart = workbook.add_chart((ChartType::Bar));
                let mut series = chart.add_series(None, None);
                series.set_name("地域");
                series.set_categories(sheet_name, 17, 6, 24, 6);
                series.set_values(sheet_name, 17, 7, 24, 7);
                user_graph_sheet.insert_chart(61, starting_col + 9, &chart);
            }
            }
            // == without n-number ==
            let mut starting_col = 28;
            {
               // -- First column --
            // Label
            user_graph_sheet.merge_range(0,starting_col, 0, starting_col + 7,
                                         "n数なし　未既婚/子供の人数/世帯年収", Some
                (&bg_highlight_red))?;
            // Marital chart
            {
                let mut chart = workbook.add_chart((ChartType::Pie));
                let mut series = chart.add_series(None, None);
                series.set_name("未既婚");
                // series.set_fill()
                series.set_fill(&chart_fill_1);
                series.set_categories(sheet_name, 25, 0, 26, 0);
                series.set_values(sheet_name, 25, 1, 26, 1);
                user_graph_sheet.insert_chart(1, starting_col, &chart);
            }
            //  Children chart
            {
                let mut chart = workbook.add_chart((ChartType::Pie));
                let mut series = chart.add_series(None, None);
                series.set_name("子供の人数");
                series.set_categories(sheet_name, 30, 0, 34, 0);
                series.set_values(sheet_name, 30, 1, 34, 1);
                user_graph_sheet.insert_chart(16, starting_col, &chart);
            }
            //  Income chart
            {
                let mut chart = workbook.add_chart((ChartType::Bar));
                let mut series = chart.add_series(None, None);
                series.set_name("世帯年収");
                series.set_categories(sheet_name, 29, 6, 42, 6);
                series.set_values(sheet_name, 29, 7, 42, 7);
                user_graph_sheet.insert_chart(31, starting_col, &chart);
            }

            // -- Second column --
            // Label
            user_graph_sheet.merge_range(0,starting_col + 9, 0, starting_col + 16,
                                         "n数なし　年代/性別/職業/地域", Some
                (&bg_highlight_red))?;
            // AgeGroup1060 chart
            {
                let mut chart = workbook.add_chart((ChartType::Pie));
                let mut series = chart.add_series(None, None);
                series.set_name("年代");
                // series.set_fill()
                series.set_fill(&chart_fill_1);
                series.set_categories(sheet_name, 1, 0, 6, 0);
                series.set_values(sheet_name, 1, 1, 6, 1);
                user_graph_sheet.insert_chart(1, starting_col + 9, &chart);
            }
            //  Gender chart
            {
                let mut chart = workbook.add_chart((ChartType::Pie));
                let mut series = chart.add_series(None, None);
                series.set_name("性別");
                series.set_categories(sheet_name, 20, 0, 21, 0);
                series.set_values(sheet_name, 20, 1, 21, 1);
                user_graph_sheet.insert_chart(16, starting_col + 9, &chart);
            }
            // AgeGroup1070 chart
            {
                let mut chart = workbook.add_chart((ChartType::Pie));
                let mut series = chart.add_series(None, None);
                series.set_name("年代");
                // series.set_fill()
                series.set_fill(&chart_fill_1);
                series.set_categories(sheet_name, 10, 0, 16, 0);
                series.set_values(sheet_name, 10, 1, 16, 1);
                user_graph_sheet.insert_chart(31, starting_col + 9, &chart);
            }
            //  Job chart
            {
                let mut chart = workbook.add_chart((ChartType::Bar));
                let mut series = chart.add_series(None, None);
                series.set_name("職業");
                series.set_categories(sheet_name, 1, 6, 12, 6);
                series.set_values(sheet_name, 1, 7, 12, 7);
                user_graph_sheet.insert_chart(46, starting_col + 9, &chart);
            }
            //  Region chart
            {
                let mut chart = workbook.add_chart((ChartType::Bar));
                let mut series = chart.add_series(None, None);
                series.set_name("地域");
                series.set_categories(sheet_name, 17, 6, 24, 6);
                series.set_values(sheet_name, 17, 7, 24, 7);
                user_graph_sheet.insert_chart(61, starting_col + 9, &chart);
            }
            }

        }
    }

    // Aggregate
    {
        let sheet_name = "aggregate";
        let tables = dataset.get_aggregate_tables()?;
        let mut sheet = workbook.add_worksheet(Some(sheet_name))?;
        let mut coord = CellCoord::new(0, 0);
        for table_with_meta in tables.into_iter() {
            for meta in table_with_meta.meta {
                sheet.write_string(coord.row, coord.col, &meta, None);
                coord.increment_row();
            }
            coord = write_table(coord.row, coord.col, table_with_meta.table, &mut sheet, &bg_normal,
                              &bg_highlight, &num_format, &perc_format)?;
            coord = CellCoord::new(0, coord.col + 2);
        }
    }

    // Crosstab - n
    {
        let sheet_name = "crosstab(n)";
        let tables = dataset.get_crosstab_tables(CrosstabType::N)?;
        let mut sheet = workbook.add_worksheet(Some(sheet_name))?;
        let mut coord = CellCoord::new(0, 0);
        for table_with_meta in tables.into_iter() {
            coord = write_table_with_meta(coord.row, coord.col, table_with_meta, &mut sheet,
            &bg_normal, &bg_highlight, &num_format, &perc_format)?;
            coord = CellCoord::new(coord.row, 0);
        }
    }

    // Crosstab - %
    {
        let sheet_name = "crosstab(%)";
        let tables = dataset.get_crosstab_tables(CrosstabType::Perc)?;
        let mut sheet = workbook.add_worksheet(Some(sheet_name))?;
        let mut coord = CellCoord::new(0, 0);
        for table_with_meta in tables.into_iter() {
            coord = write_table_with_meta(coord.row, coord.col, table_with_meta, &mut sheet,
                                          &bg_normal, &bg_highlight, &num_format, &perc_format)?;
            coord = CellCoord::new(coord.row, 0);
        }
    }

    workbook.close()?;

    let temp_file_path = std::env::current_dir()?.join(&temp_file_name);
    println!("{:?}", temp_file_path);
    std::fs::copy(temp_file_path, output_path)?;
    clean_file_or_current_dir(Some(&temp_file_name));

    println!("Done! Xlsx file created.");
    // Ok(format!("./{}", output_name))
    Ok(())

}

fn clean_file_or_current_dir(file: Option<&str>) -> std::io::Result<()> {
    let mut current_dir = std::env::current_dir()?;
    match file {
        Some(file_name) => {
            current_dir.push(file_name);
            std::fs::remove_file(current_dir)?
        },
        None => {
            for entry in std::fs::read_dir(current_dir)? {
                if let Ok(entry) = entry {
                    if !entry.file_name().into_string().unwrap().contains(".xlsx") {
                        continue;
                    } else {
                        std::fs::remove_file(entry.path());
                    }
                }
            }
        }
    };
    Ok(())
}

fn write_table(
    starting_row: WorksheetRow,
    starting_col: WorksheetCol,
    table: Table,
    worksheet: &mut Worksheet,
    bg_normal: &Format,
    bg_highlight: &Format,
    num_format: &Format,
    perc_format: &Format) -> Result<CellCoord, RustlyzerError>
{
    write_table_with_meta(
        starting_row,
        starting_col,
        TableWithMeta::with_special_case(SpecialCase::None, table),
        worksheet,
        bg_normal,
        bg_highlight,
        num_format,
        perc_format
    )
}


fn write_table_with_meta(
    starting_row: WorksheetRow,
    starting_col: WorksheetCol,
    table_with_meta: TableWithMeta,
    worksheet: &mut Worksheet,
    bg_normal: &Format,
    bg_highlight: &Format,
    num_format: &Format,
    perc_format: &Format) -> Result<CellCoord, RustlyzerError>
{
    let special_case = table_with_meta.special_case;
    let mut coord =
        if let SpecialCase::SpaceAndHighlightOn4 = special_case { CellCoord::new (starting_row + 1,
                                                                                  starting_col) }
        else { CellCoord::new (starting_row, starting_col) };
    let mut max_row_content: WorksheetRow = 0;
    for (col_index, col) in table_with_meta.table.cols.iter().enumerate() {
        let col_no = coord.col + col_index as u16;
        worksheet.write_string(
            coord.row,
            col_no,
            &col.header.text,
            if col.header.highlight {
                Some(&bg_highlight)
            } else {
                Some(&bg_normal)
            },
        );
        for (content_index, val) in col.contents.iter().enumerate() {
            if let Ok(num) = val.replace("%","").parse::<f64>() {
                if val.contains("%") {
                    worksheet.write_number((coord.row + content_index as u32 + 1) as u32, col_no,
                                           num,
                                           Some(perc_format));
                } else {
                    worksheet.write_number((coord.row + content_index as u32 + 1) as u32, col_no, num,
                                           Some(num_format));
                }
            } else {
                if let SpecialCase::SpaceAndHighlightOn4 = special_case {
                    if col_index == 3 {
                        worksheet.write_string((coord.row + content_index as u32 + 1) as u32, col_no, val,
                                               Some(&bg_highlight));
                        continue;
                    }
                }
                worksheet.write_string((coord.row + content_index as u32 + 1) as u32, col_no, val,
                                       None);
            }

        }
        max_row_content = if col.contents.len() as u32 > max_row_content { col.contents.len() as
            u32 } else
        {max_row_content};
        if let Some(footer_text) = &col.footer {
            if let Ok(num) = footer_text.replace("%","").parse::<f64>() {
                if footer_text.contains("%") {
                    worksheet.write_number((coord.row + max_row_content + 1), col_no,
                                           num, Some(perc_format));
                } else {
                    worksheet.write_number((coord.row + max_row_content + 1), col_no, num,
                                     Some(num_format));
                }
            } else {
                worksheet.write_string((coord.row + max_row_content + 1), col_no, footer_text,
                                       None);
            }
        }
    }
    coord = CellCoord::new(coord.row + 1 + max_row_content, coord.col + table_with_meta.table
        .cols.len() as
        u16);
    Ok(coord)
}

struct CellCoord {
    pub row: WorksheetRow,
    pub col: WorksheetCol
}

impl CellCoord {
    fn new(row: WorksheetRow, col: WorksheetCol) -> Self {
        CellCoord {row, col}
    }

    fn increment_row(&mut self) {
        self.row += 1;
    }
}