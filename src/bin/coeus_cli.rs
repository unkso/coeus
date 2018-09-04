#[macro_use] extern crate serde_derive;
extern crate docopt;
extern crate libcoeus;

use docopt::Docopt;

const USAGE: &'static str = "\
Unknown Soldiers Coeus Tool

Used to identify likely members to the Unknown Soldiers clan.

Usage:
  coeus cohort new <cohort-name>
  coeus cohort delete <cohort-name>
  coeus scrape <cohort-name> <max-users> <start-pos>
  coeus identify <cohort-name> <file-name> [--batch-size=<n>]

Options:
  -h --help         Show this screen.
  --version         Show version.
  --batch-size<n>   Number of users to return
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_cohort_name: String,
    arg_max_users: u32,
    arg_start_pos: u32,
    arg_file_name: String,
    flag_batch_size: Option<u32>,
    cmd_cohort: bool,
    cmd_scrape: bool,
    cmd_identify: bool,
    cmd_new: bool,
    cmd_delete: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);
}
