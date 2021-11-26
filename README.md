# golden-dawn
A daily directory maker.

## binary (for Windows)
See golden-dawn/bin directory.

## Usage
Edit config.toml and run golden.exe.
Using with Task Scheduler will enable auto run on suitable trigger.

### config.toml

| Key | Meaning |
| ------------- | ------------- |
| parent_dir | The path of parent dir. golden-dawn creates daily directory in parent dir.  |
| date_format | Name format of daily directory. See also https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers . |
| days_to_move | If elapsed days of directory exceeds this value, golden-dawn move that directory to "old" directory. |
| days_to_remove | If elapsed days of directory exceeds this value, golden-dawn remove that directory from "old" directory. |
