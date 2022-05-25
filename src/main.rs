use clap::Parser;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Result, Write};

#[derive(Parser, Debug)]
#[clap(author="Haozhan Shi", version, about, long_about = None)]
struct Args {
    /// Input file path
    #[clap(short, long, help = "Input file path")]
    input: String,

    /// Output file path
    #[clap(short, long, help = "Output file path")]
    output: Option<String>,

    /// Show statistics of lat and lon
    #[clap(short, long, parse(from_flag), help = "Show statistics of lat and lon")]
    stats: bool,
}

fn write_file(content: &Vec<(f64, f64)>, filename: &OsStr) -> Result<()> {
    let mut file = File::create(filename)?;
    for (x, y) in content {
        writeln!(file, "{} {}", x, y)?;
    }
    Ok(())
}

fn convert(infile: &OsStr, outfile: Option<&OsStr>, stats: bool) -> std::io::Result<()> {
    // File handle
    let mut in_path = std::path::PathBuf::from(infile);
    let r = File::open(&in_path).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(r);

    let mut nb_nodes = 0;
    let mut sum_lon = 0.;
    let mut sum_lat = 0.;

    let mut x_nodes: Vec<f64> = Vec::new();
    let mut y_nodes: Vec<f64> = Vec::new();

    for obj in pbf.par_iter().map(core::result::Result::unwrap) {
        if let osmpbfreader::OsmObj::Node(node) = obj {
            nb_nodes += 1;
            let x = node.lat();
            let y = node.lon();
            sum_lat += x;
            sum_lon += y;
            x_nodes.push(x);
            y_nodes.push(y);
        }
    }

    // Optional print all nodes stats
    if stats {
        println!(
            "Total {} nodes, mean coord: {}, {}",
            nb_nodes,
            sum_lat / nb_nodes as f64,
            sum_lon / nb_nodes as f64
        );
    }

    // Collect all lat and lon into Vec<(f64, f64)>
    let mut all_nodes = x_nodes
        .iter()
        .cloned()
        .zip(y_nodes.iter().cloned())
        .collect::<Vec<(f64, f64)>>();

    // Sort along lat
    all_nodes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Convert output file path if provided, otherwise use input file path
    let out_path = match outfile {
        Some(outfile) => outfile,
        None => {
            in_path.set_extension("txt");
            in_path.as_ref()
        }
    };

    match write_file(&all_nodes, out_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    println!("Writen to file: {:#?}", out_path);

    Ok(())
}

fn option_string_to_os_str(input: &Option<String>) -> Option<&OsStr> {
    match input {
        Some(s) => Some(OsStr::new(s)),
        None => None,
    }
}

fn arg_parse(args: Args) {
    let infile = args.input;
    let outfile = option_string_to_os_str(&args.output);
    let stats = args.stats;

    match convert(OsStr::new(&infile), outfile, stats) {
        Ok(_) => (),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn main() {
    let args = Args::parse();
    arg_parse(args);
}
