//! This example prints simple information on the streams contained
//! in a matroska file.

// rust-av crates
extern crate av_data as data;
extern crate av_format as format;

// Matroska demuxer
extern crate matroska;

// CLI crates
extern crate clap;

use std::fs::File;
use std::path::Path;

use data::params::MediaKind;
use format::buffer::AccReader;
use format::demuxer::Context;

use matroska::demuxer::MkvDemuxer;

use clap::{App, Arg};

fn main() {
    // Set up CLI configuration and input parameters
    let matches = App::new("streams-info")
        .about("Gets information on audio and video streams")
        .arg(
            Arg::with_name("path")
                .help("Sets the matroska file to analyze")
                .short("i")
                .long("input")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // Get the path to the matroska file
    let path = matches.value_of("path").map(|s| Path::new(s)).unwrap();

    // Open the matroska file
    let reader = File::open(path).unwrap();

    // Create a buffer of size 4096MB to contain matroska data
    let ar = AccReader::with_capacity(4 * 1024, reader);

    // Set the type of demuxer, in this case, a matroska demuxer
    let mut demuxer = Context::new(Box::new(MkvDemuxer::new()), Box::new(ar));

    // Read matroska headers
    demuxer
        .read_headers()
        .expect("Cannot parse the format headers");

    // Iterate over the streams contained in a matroska file
    for stream in &demuxer.info.streams {
        // Print simple information on video and audio streams
        match stream.params.kind {
            Some(MediaKind::Video(ref info)) => {
                println!("Video streams information\n");
                println!("width: {}", info.width);
                println!("height: {}", info.height);
            }
            Some(MediaKind::Audio(ref info)) => {
                println!("\nAudio streams information\n");
                println!("rate: {}", info.rate);
            }
            _ => {}
        }
    }
}
