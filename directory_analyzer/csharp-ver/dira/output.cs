using dira.types;
using ClosedXML.Excel;

namespace dira
{

    public static class B
    {
        public static string Output(Args args, AnalyzedInfo analyzedInfo)
        {
            if (args.ExportXLSX != null)
            {

                var workbook = CreateWorkbook(analyzedInfo);
                var path = WriteWorkbook(args.ExportXLSX, workbook);
                string format = "Info saved to {0}";
                return string.Format(format, path);

            }
            else
            {
                return string.Format("{0}", analyzedInfo);
            }
        }

        static XLWorkbook CreateWorkbook(AnalyzedInfo info)
        {
            var workbook = new XLWorkbook();
            var gen_worksheet = workbook.AddWorksheet("General Info");
            var cur_col = 1;
            gen_worksheet.Cell(1, cur_col).Value = "Found dirs";
            gen_worksheet.Cell(1, cur_col).Style.Font.SetBold();
            gen_worksheet.Cell(2, cur_col).Value = info.FoundDirs;
            gen_worksheet.Column(cur_col).AdjustToContents();
            cur_col += 1;
            gen_worksheet.Cell(1, cur_col).Value = "Found file";
            gen_worksheet.Cell(1, cur_col).Style.Font.SetBold();
            gen_worksheet.Cell(2, cur_col).Value = info.FoundFiles;
            gen_worksheet.Column(cur_col).AdjustToContents();
            cur_col += 1;
            gen_worksheet.Cell(1, cur_col).Value = "Total bytes";
            gen_worksheet.Cell(1, cur_col).Style.Font.SetBold();
            gen_worksheet.Cell(2, cur_col).Value = info.TotalBytes;
            gen_worksheet.Column(cur_col).AdjustToContents();
            cur_col += 1;
            if (info.FoundSymlinks is SymlinkInfo symlinkInfo)
            {
                gen_worksheet.Cell(1, cur_col).Value = "Found symlinks";
                gen_worksheet.Cell(1, cur_col).Style.Font.SetBold();
                gen_worksheet.Cell(2, cur_col).Value = symlinkInfo.FoundSymlinks;
                gen_worksheet.Column(cur_col).AdjustToContents();
                cur_col += 1;
                gen_worksheet.Cell(1, cur_col).Value = "File symlinks";
                gen_worksheet.Cell(1, cur_col).Style.Font.SetBold();
                gen_worksheet.Cell(2, cur_col).Value = symlinkInfo.GetFileSymlinks();
                gen_worksheet.Column(cur_col).AdjustToContents();
                cur_col += 1;
                gen_worksheet.Cell(1, cur_col).Value = "Dir symlinks";
                gen_worksheet.Cell(1, cur_col).Style.Font.SetBold();
                gen_worksheet.Cell(2, cur_col).Value = symlinkInfo.GetDirSymlinks();
                gen_worksheet.Column(cur_col).AdjustToContents();
            }

            if (info.FileInfo is Dictionary<FileExtension, FileTypeInfo> file_info)
            {
                var fi_worksheet = workbook.AddWorksheet("File Info");
                cur_col = 1;
                var down_offset = 1;
                fi_worksheet.Cell(down_offset, cur_col).Value = "File Type";
                fi_worksheet.Cell(down_offset, cur_col).Style.Font.SetBold();
                cur_col += 1;
                fi_worksheet.Cell(down_offset, cur_col).Value = "Num files";
                fi_worksheet.Cell(down_offset, cur_col).Style.Font.SetBold();
                cur_col += 1;
                fi_worksheet.Cell(down_offset, cur_col).Value = "% of total files";
                fi_worksheet.Cell(down_offset, cur_col).Style.Font.SetBold();
                cur_col += 1;
                fi_worksheet.Cell(down_offset, cur_col).Value = "Total size of files(bytes)";
                fi_worksheet.Cell(down_offset, cur_col).Style.Font.SetBold();
                cur_col += 1;
                fi_worksheet.Cell(down_offset, cur_col).Value = "% of total bytes";
                fi_worksheet.Cell(down_offset, cur_col).Style.Font.SetBold();
                cur_col += 1;
                fi_worksheet.Cell(down_offset, cur_col).Value = "Largest file";
                fi_worksheet.Cell(down_offset, cur_col).Style.Font.SetBold();
                cur_col += 1;
                fi_worksheet.Cell(down_offset, cur_col).Value = "Largest file size(bytes)";
                fi_worksheet.Cell(down_offset, cur_col).Style.Font.SetBold();
                cur_col += 1;
                fi_worksheet.Cell(down_offset, cur_col).Value = "Smallest file";
                fi_worksheet.Cell(down_offset, cur_col).Style.Font.SetBold();
                cur_col += 1;
                fi_worksheet.Cell(down_offset, cur_col).Value = "Smallest file size(bytes)";
                fi_worksheet.Cell(down_offset, cur_col).Style.Font.SetBold();
                var max_col = cur_col + 1;

                var tmp = file_info.ToArray();
                for (int i = 0; i < file_info.Count; i++)
                {
                    var k = i + 1 + down_offset;
                    cur_col = 1;
                    var f_info = tmp[i].Value;
                    //File type
                    fi_worksheet.Cell(k, cur_col).Value = tmp[i].Key.Ext;
                    cur_col += 1;
                    //Num Files
                    fi_worksheet.Cell(k, cur_col).Value = f_info.NumFiles;
                    cur_col += 1;
                    (var per_tot_file, var per_tot_size) = f_info.PercentagesInString();
                    //% of total files
                    fi_worksheet.Cell(k, cur_col).Value = per_tot_file.Percent;
                    cur_col += 1;
                    //Total size of files
                    fi_worksheet.Cell(k, cur_col).Value = f_info.SizeInBytes;
                    cur_col += 1;
                    //% of total size
                    fi_worksheet.Cell(k, cur_col).Value = per_tot_size.Percent;
                    cur_col += 1;
                    //Largest file path
                    fi_worksheet.Cell(k, cur_col).Value = f_info.LargestFile.Path;
                    cur_col += 1;
                    //Largest file size
                    fi_worksheet.Cell(k, cur_col).Value = f_info.LargestFile.Size;
                    cur_col += 1;
                    //Smallest file path
                    fi_worksheet.Cell(k, cur_col).Value = f_info.SmallestFile.Path;
                    cur_col += 1;
                    //Smallest file size
                    fi_worksheet.Cell(k, cur_col).Value = f_info.SmallestFile.Size;
                }

                foreach (var i in Enumerable.Range(1, max_col + 1))
                {
                    fi_worksheet.Column(i).AdjustToContents();
                }
            }

            return workbook;
        }

        static string WriteWorkbook(string path, XLWorkbook workbook)
        {
            var final_path = path;
            if (Path.GetExtension(path) != ".xlsx")
            {
                final_path += ".xlsx";
            }
            workbook.SaveAs(final_path);
            return final_path;
        }
    }
}
