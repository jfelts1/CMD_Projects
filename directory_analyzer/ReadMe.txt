This program is used to analyze a directory and tell you about the contents.

Usage: dira.exe [OPTIONS] <PATH_TO_ANALYZE>

Arguments:
  <PATH_TO_ANALYZE>
          Requied path to analyze

Options:
  -n, --no-recurse
          Just look at the files in the current directory

  -f, --file-info
          Output info about what file types were found, such as type and size

  -s, --follow-symlinks
          Follow symlinks as if they were a normal object.
          
          NOTE: Due to the need to track found files and directories to prevent counting the same entry multiple times, this has a noticable performance impact. Due to this unless you need to actually follow symlinks it is recomended to not use this flag.

  -c, --count-symlinks
          Counts symlinks found but does not follow them

  -v, --verbose
          Print each object as it is found

  -e, --export-xlsx <EXPORT_XLSX>
          Export info to a xlsx file at the location specified

  -p, --full-path
          When displaying paths should they be printed in full

  -i, --ignore-entries <IGNORE_ENTRIES>
          Comma seperated list of directories and files that will be not included in the analysis

  -u, --updates <UPDATES>
          Provides periodic updates about how many entries have been analyized. Value is seconds between updates

  -t, --time
          Displays the time the program took to run to stdout

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
