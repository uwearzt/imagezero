// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use std::fs;
    use std::path::PathBuf;

    use std::convert::TryInto;

    #[test]
    fn compat() {
        setup();
        // read images, compress with rust and cpp implemtation
        // uncompress with rust and cpp implementation

        if let Ok(entries) = fs::read_dir("./images") {
            for entry in entries {
                if let Ok(file) = entry {
                    if let Ok(metadata) = file.metadata() {
                        if metadata.is_file() {
                            let path = file.path();
                            if path.extension().unwrap() == "ppm" {
                                read_img(path);
                            }
                        }
                    }
                }
            }
        }
        teardown();
    }

    fn setup() {
        unsafe {
            imagezero_sys::IZ_initEncodeTable();
            imagezero_sys::IZ_initDecodeTable();
        }
    }
    fn teardown() {}

    fn read_img(path: PathBuf) {
        println!("{:?}", path);

        // input image
        let img = image::open(path).unwrap();
        if img.color() != image::ColorType::Rgb8 {
            println!("Wrong image format");
        }

        // -------------------------------------------------------------------
        // encode cpp
        let rgb = img.into_rgb8();
        let rgb2 = rgb.clone();
        let (width, height) = rgb.dimensions();

        // output buffer
        let memsize: usize = (height * width * 4 + 33).try_into().unwrap();
        let mut buf: Vec<u8> = Vec::with_capacity(memsize);

        let p_in = imagezero_sys::IZ_Image {
            m_width: width as i32,
            m_height: height as i32,
            m_data: rgb.into_raw().as_mut_ptr(),
            m_spl: (width * 3) as isize,
        };
        
        unsafe {
            let p_buf = buf.as_mut_ptr();

            let p_end = imagezero_sys::IZ_encodeImage(&p_in, p_buf);
            let retsize = p_end.offset_from(p_buf);

            buf.set_len(retsize as usize);
        }
        // encode rust
        let encoded = imagezero::encode(rgb2, width, height);
        let (_r_rgb, _r_width, _r_height) = imagezero::decode(encoded);

        // -------------------------------------------------------------------
        // decode cpp
        let x: *mut u8 = &mut 0;
        let mut img = imagezero_sys::IZ_Image {
            m_width: 0,
            m_height: 0,
            m_data: x,
            m_spl: 0,
        };

        unsafe {
            let p_buf = buf.as_mut_ptr();
            imagezero_sys::IZ_decodeImageSize(&mut img, p_buf);

            let datasize = img.m_width * img.m_height * 3;
            let mut outbuf: Vec<u8> = Vec::with_capacity(datasize as usize);
            img.m_data = outbuf.as_mut_ptr();
            imagezero_sys::IZ_decodeImage(&mut img, p_buf);
            outbuf.set_len(datasize as usize);
        }
        assert_eq!(p_in.m_width, img.m_width);
        assert_eq!(p_in.m_height, img.m_height);
        assert_eq!(p_in.m_spl, img.m_spl);
        //assert_eq!(p_in.m_data, img.m_data);
        assert_eq!(0, unsafe {
            libc::memcmp(
                p_in.m_data as *mut libc::c_void,
                img.m_data as *mut libc::c_void,
                img.m_spl as usize,
            )
        });
    }
}
