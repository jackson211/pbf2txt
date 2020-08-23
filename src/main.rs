#[macro_use]
extern crate log;
extern crate env_logger;
extern crate osmpbfreader;
use std::path::Path;

// use std::fmt;
use std::fs::File;
use std::io::prelude::*;

fn write_file(content: &Vec<(f64, f64)>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    for (x, y) in content {
        writeln!(file, "{} {}", x, y)?;
    }
    Ok(())
}

fn count(filename: &std::ffi::OsStr) -> (Vec<(f64, f64)>, f64, f64) {
    let r = std::fs::File::open(&std::path::Path::new(filename)).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(r);
    let mut nb_nodes = 0;
    let mut sum_lon = 0.;
    let mut sum_lat = 0.;

    let mut mean_x = 0.0;
    let mut mean_y = 0.0;
    let mut c = 0.0;
    let mut m2 = 0.0;

    let mut x_nodes: Vec<f64> = Vec::new();
    let mut y_nodes: Vec<f64> = Vec::new();

    for obj in pbf.par_iter().map(Result::unwrap) {
        info!("{:?}", obj);
        if let osmpbfreader::OsmObj::Node(node) = obj {
            nb_nodes += 1;
            let x = node.lon();
            let y = node.lat();
            sum_lon += node.lon();
            sum_lat += node.lat();

            // n += 1;
            let dx = x - mean_x;
            mean_x += dx / f64::from(nb_nodes);
            mean_y += (y - mean_y) / f64::from(nb_nodes);
            c += dx * (y - mean_y);

            let dx2 = x - mean_x;
            m2 += dx * dx2;
            x_nodes.push(x);
            y_nodes.push(y);
        }
    }

    println!(
        "Total {} nodes, mean coord: {}, {}",
        nb_nodes,
        sum_lat / nb_nodes as f64,
        sum_lon / nb_nodes as f64,
    );

    let cov = c / f64::from(nb_nodes - 1);
    let var = m2 / f64::from(nb_nodes - 1);
    assert!(var >= 0.0);
    let slope: f64 = cov / var;
    let intercept = mean_y - slope * mean_x;

    let mut all_nodes = x_nodes
        .iter()
        .cloned()
        .zip(y_nodes.iter().cloned())
        .collect::<Vec<(f64, f64)>>();

    all_nodes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Convert file path
    let path = Path::new(filename);
    let stem = path.file_stem().unwrap_or(std::ffi::OsStr::new(""));
    let v: Vec<&str> = stem.to_str().unwrap().split(".").collect();
    let mut outfile = v[0].trim().to_string();
    outfile.push_str(".txt");

    match write_file(&all_nodes, &outfile) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    (all_nodes, intercept, slope)
}

fn main() {
    env_logger::init();
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() == 2 {
        println!("counting objects...");
        count(&args[1]);
    } else {
        println!("Input file path not given");
    }
}
