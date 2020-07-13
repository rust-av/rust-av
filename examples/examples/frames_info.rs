//! This example prints information on every video and audio frames
//! contained in a matroska file.

// rust-av crates
extern crate av_codec as codec;
extern crate av_data as data;
extern crate av_format as format;

// Matroska demuxer
extern crate matroska;

// Audio decoders
extern crate av_vorbis as vorbis;
extern crate libopus as opus;

// Video decoders
extern crate libvpx as vpx;

// CLI crates
extern crate clap;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use codec::common::CodecList;
use codec::decoder::Codecs as DecCodecs;
use codec::decoder::Context as DecContext;
use data::frame::{ArcFrame, MediaKind};
use format::buffer::AccReader;
use format::demuxer::{Context, Event};

use matroska::demuxer::MkvDemuxer;

use opus::decoder::OPUS_DESCR;
use vorbis::decoder::VORBIS_DESCR;
use vpx::decoder::VP9_DESCR;

use clap::{App, Arg};

// This function decodes a single frame using the most appropriate decoder
fn decode_single_frame(
    demuxer: &mut Context,
    decoders: &mut HashMap<isize, DecContext>,
) -> Result<Option<ArcFrame>, String> {
    // The demuxer reads which event has occurred
    match demuxer.read_event() {
        // If a new packet has been found, decode it
        Ok(event) => match event {
            Event::NewPacket(pkt) => {
                // Choose the right decoder for the packet
                if let Some(decoder) = decoders.get_mut(&pkt.stream_index) {
                    decoder.send_packet(&pkt).unwrap();
                    Ok(decoder.receive_frame().ok())
                } else {
                    // If a packet cannot be decoded, it will be skipped
                    println!("Skipping packet at index {}", pkt.stream_index);
                    Ok(None)
                }
            }
            // When the EOF is reached, the decoding process is stopped
            Event::Eof => {
                println!("EOF reached.");
                Err("EOF reached".to_owned())
            }
            _ => {
                // If an unsupported event occurs,
                // the decoding process is stopped
                println!("Unsupported event {:?}", event);
                Err("Unsupported event".to_owned())
            }
        },
        Err(err) => {
            // If there are no more events, the decoding process is stopped
            println!("No more events {:?}", err);
            Err("No more events".to_owned())
        }
    }
}

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

    // Set the list of supported decoders
    let decoders = DecCodecs::from_list(&[VP9_DESCR, OPUS_DESCR, VORBIS_DESCR]);

    // Save the context of each decoder (its configuration)
    let mut decs: HashMap<isize, DecContext> = HashMap::with_capacity(2);

    // Iterate over the streams contained in a matroska file
    for stream in &demuxer.info.streams {
        // Get the id of the codec associated to the stream
        if let Some(ref codec_id) = stream.params.codec_id {
            // Get the right decoder from the decoder list using the id of a codec
            if let Some(mut ctx) = DecContext::by_name(&decoders, codec_id) {
                // Add stream extradata to the decoder
                if let Some(ref extradata) = stream.params.extradata {
                    ctx.set_extradata(extradata);
                }
                // Configure a decoder
                ctx.configure().expect("Codec configure failed");
                // Save the context of a decoder
                decs.insert(stream.index as isize, ctx);
            }
        }
    }

    let mut frame_idx = 0;
    // Iterate over the decoded frames
    while let Ok(data) = decode_single_frame(&mut demuxer, &mut decs) {
        // Check if the decoding process of a frame went well
        if let Some(frame) = data {
            // Print the kind of a decoded frame
            match frame.kind {
                MediaKind::Video(_) => {
                    println!("Frame {} is a video frame\n", frame_idx);
                }
                MediaKind::Audio(_) => {
                    println!("Frame {} is an audio frame\n", frame_idx);
                }
            }
            // Print the content of a frame
            println!("Decoded {:?}\n\n", frame);
            frame_idx += 1;
        }
    }
}
