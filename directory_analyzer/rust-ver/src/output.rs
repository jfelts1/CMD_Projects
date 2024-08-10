use std::path::Path;

use anyhow::Error;
use rust_xlsxwriter::{Format, Workbook, Worksheet};

use crate::{AnalyzedInfo, Args};

///Returns a string analyzed_info based on how args is configured
pub fn output(args: &Args, analyzed_info: Result<AnalyzedInfo, Error>) -> anyhow::Result<String> {
    match analyzed_info {
        Ok(info) => match args.export_xlsx() {
            Some(path) => {
                let workbook = create_workbook(&info)?;
                let path = write_workbook(path, workbook)?;
                let out = format!("Info saved to {}", &path.to_string_lossy());
                Ok(out)
            }
            None => Ok(format!("{info}")),
        },
        Err(e) => Err(e),
    }
}

fn write_workbook(
    path: &Path,
    mut workbook: Workbook,
) -> Result<std::path::PathBuf, Error> {
    let mut path = path.to_path_buf();
    if path.extension().is_none() {
        path.set_extension("xlsx");
    }
    workbook.save(&path)?;
    Ok(path)
}

fn create_workbook(info: &AnalyzedInfo) -> Result<Workbook, Error> {
    let mut workbook = Workbook::new();
    let bold_format = Format::new().set_bold();
    let mut worksheet = Worksheet::new();
    worksheet.set_name("General Info")?;
    let mut cur_col = 0;
    worksheet.write_with_format(0, cur_col, "Found dirs", &bold_format)?;
    worksheet.write(1, cur_col, info.found_dirs())?;
    cur_col += 1;
    worksheet.write_with_format(0, cur_col, "Found files", &bold_format)?;
    worksheet.write(1, cur_col, info.found_files())?;
    cur_col += 1;
    if let Some(symlink_info) = info.found_symlinks() {
        worksheet.write_with_format(0, cur_col, "Found symlinks", &bold_format)?;
        worksheet.write(1, cur_col, symlink_info.found_symlinks())?;
        cur_col += 1;
        worksheet.write_with_format(0, cur_col, "File symlinks", &bold_format)?;
        worksheet.write(1, cur_col, symlink_info.file_symlinks())?;
        cur_col += 1;
        worksheet.write_with_format(0, cur_col, "Dir symlinks", &bold_format)?;
        worksheet.write(1, cur_col, symlink_info.dir_symlinks())?;
        // cur_col += 1;
    }
    worksheet.autofit();
    workbook.push_worksheet(worksheet);
    if let Some(file_info) = info.file_info() {
        let mut fi_worksheet =Worksheet::new();
        fi_worksheet.set_name("File Info")?;
        let down_offset = 0;
        let tmp: Vec<_> = file_info.iter().enumerate().collect();
        fi_worksheet.write_with_format(down_offset, 0, "File type", &bold_format)?;
        fi_worksheet.write_with_format(down_offset, 1, "Num files", &bold_format)?;
        fi_worksheet.write_with_format(down_offset, 2, "Total size of files(bytes)", &bold_format)?;
        fi_worksheet.write_with_format(down_offset, 3, "Largest file", &bold_format)?;
        fi_worksheet.write_with_format(down_offset, 4, "Largest file size(bytes)", &bold_format)?;
        fi_worksheet.write_with_format(down_offset, 5, "Smallest file", &bold_format)?;
        fi_worksheet.write_with_format(down_offset, 6, "Smallest file size(bytes)", &bold_format)?;

        for (i, (f_type, f_info)) in tmp {
            let i = i as u32 + down_offset + 1;
            fi_worksheet.write(i, 0, f_type)?;
            fi_worksheet.write(i, 1, f_info.num_files())?;
            fi_worksheet.write(i, 2, f_info.size_in_bytes())?;
            fi_worksheet.write(i, 3, f_info.largest_file().path().to_string_lossy())?;
            fi_worksheet.write(i, 4, f_info.largest_file().size())?;
            fi_worksheet.write(i, 5, f_info.smallest_file().path().to_string_lossy())?;
            fi_worksheet.write(i, 6, f_info.smallest_file().size())?;
        }
        fi_worksheet.autofit();
        workbook.push_worksheet(fi_worksheet);
    }
    Ok(workbook)
}
