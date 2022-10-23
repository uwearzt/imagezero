// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgGroup};


use std::fs::File;
use std::io::Read;

fn main() {
    let parms = App::new("imagezero")
        .version(crate_version!())
        .about("compress/decompress imagezero file")
        .author(crate_authors!())
        .group(
            ArgGroup::with_name("action")
                .required(true)
                .args(&["compress", "decompress"]),
        )
        .arg(
            Arg::with_name("compress")
                .short("c")
                .long("compress")
                .help("compress ppm to iz"),
        )
        .arg(
            Arg::with_name("decompress")
                .short("d")
                .long("decompress")
                .help("decompress iz to ppm"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Input image")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Output image")
                .required(true)
                .index(2),
        )
        .get_matches();

    let mut infile = "";
    let mut outfile = "";

    if let Some(file) = parms.value_of("INPUT") {
        infile = file;
    }
    if let Some(file) = parms.value_of("OUTPUT") {
        outfile = file;
    }
    if parms.is_present("compress") {
        compress(infile, outfile);
    }
    if parms.is_present("decompress") {
        decompress(infile, outfile);
    }

    fn compress(ppm: &str, iz: &str) {
        println!("compress {} to {}", ppm, iz);

        // input image
        let img = image::open(ppm).unwrap();
        if img.color() != image::ColorType::Rgb8 {
            println!("Wrong image format");
        }
        let rgb = img.into_rgb8();
        let (_width, _height) = rgb.dimensions();
    }

    fn decompress(iz: &str, ppm: &str) {
        println!("decompress {} to {}", iz, ppm);

        // input image
        let mut iz = File::open(iz).unwrap();
        let mut buffer = Vec::new();
        iz.read_to_end(&mut buffer).unwrap();
    }
}
