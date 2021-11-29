use dacapo_latency_dump_hdrh::DaCapoLatencyDump;
use glob::glob;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rayon::prelude::*;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut entries: Vec<PathBuf> = (&args[1..])
        .iter()
        .map(|arg| {
            let entries: Result<Vec<PathBuf>, _> =
                glob(&format!("{}/**/dacapo-latency-usec-*.csv", arg))
                    .expect("Failed to read glob pattern")
                    .collect();
            entries.expect("Failed to read entries")
        })
        .collect::<Vec<Vec<PathBuf>>>()
        .concat();
    let pb = ProgressBar::new(entries.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .progress_chars("#>-"),
    );
    entries.par_iter_mut().progress_with(pb).for_each(|entry| {
        let dld = DaCapoLatencyDump::new(entry);
        entry.set_extension("hdrh");
        dld.save_hdrh(entry)
    });
}
