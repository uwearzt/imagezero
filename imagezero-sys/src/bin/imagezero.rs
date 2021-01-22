// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgGroup};

use std::convert::TryInto;

use std::fs::File;
use std::io::Read;
use std::io::Write;

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
        let (width, height) = rgb.dimensions();

        // output buffer
        let memsize: usize = (height * width * 4 + 33).try_into().unwrap();
        let mut outbuf: Vec<u8> = Vec::with_capacity(memsize);

        unsafe {
            let p_in = imagezero_sys::IZ_Image {
                m_width: width as i32,
                m_height: height as i32,
                m_data: rgb.into_raw().as_mut_ptr(),
                m_spl: (width * 3) as isize,
            };
            let p_out = outbuf.as_mut_ptr();

            imagezero_sys::IZ_initEncodeTable();

            let p_end = imagezero_sys::IZ_encodeImage(&p_in, p_out);
            let retsize = p_end.offset_from(p_out);

            let mut iz = File::create(iz).unwrap();
            outbuf.set_len(retsize as usize);
            let _res = iz.write_all(outbuf.as_slice()).unwrap();
        }
    }

    fn decompress(iz: &str, ppm: &str) {
        println!("decompress {} to {}", iz, ppm);

        // input image
        let mut iz = File::open(iz).unwrap();
        let mut buffer = Vec::new();
        iz.read_to_end(&mut buffer).unwrap();

        let p_buf = buffer.as_mut_ptr();

        let x: *mut u8 = &mut 0;
        // output image
        let mut img = imagezero_sys::IZ_Image {
            m_width: 0,
            m_height: 0,
            m_data: x,
            m_spl: 0,
        };

        unsafe {
            imagezero_sys::IZ_initDecodeTable();

            imagezero_sys::IZ_decodeImageSize(&mut img, p_buf);

            let datasize = img.m_width * img.m_height * 3;
            let mut outbuf: Vec<u8> = Vec::with_capacity(datasize as usize);
            img.m_data = outbuf.as_mut_ptr();
            imagezero_sys::IZ_decodeImage(&mut img, p_buf);
            outbuf.set_len(datasize as usize);

            let mut ppm = File::create(ppm).unwrap();
            // Write PPM header
            ppm.write(b"P6\n").unwrap();
            ppm.write(img.m_width.to_string().as_bytes()).unwrap();
            ppm.write(b" ").unwrap();
            ppm.write(img.m_height.to_string().as_bytes()).unwrap();
            ppm.write(b"\n").unwrap();
            ppm.write(b"255\n").unwrap();

            ppm.write_all(outbuf.as_slice()).unwrap();
        }
    }
}
