// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#![crate_name = "imagezero"]
#![crate_type = "lib"]

pub mod compress;

use image::RgbImage;

pub fn encode(_pic: RgbImage, _width: u32, _height: u32) -> Vec<u8> {
    println!("encode");
    vec!(1,2)
}

pub fn decode(_buf: Vec<u8>) -> (RgbImage, u32, u32) {
    println!("decode");

    (RgbImage::new(32,32), 1, 2)
}