#[cfg(test)]
mod test {
    //https://stackoverflow.com/questions/30003921/how-can-i-locate-resources-for-testing-with-cargo
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn can_load_test_files() {
        let path = test_resource("read_test.txt");

        let mut f = File::open(path).expect("could not open test resource file");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("could not read from test resource file");

        assert_eq!("hello world!", &contents);
    }

    fn test_resource(resource_path: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test");
        path.push(resource_path);
        path
    }

    #[test]
    fn can_read_qr_code_contents_from_generated_image() {
        use quircs::*;

        let mut decoder = Quirc::default();
        let img = image::open(test_resource("table top.qr.png")).expect("could not open sample qr image");
        let img_gray = img.into_luma8();

        let codes: Vec<_> = decoder.identify(img_gray.width() as usize, img_gray.height() as usize, &img_gray).collect();

        assert_eq!(codes.len(), 1);
        let decoded = codes[0].as_ref().expect("could not extract qr code from sample image").decode().expect("could not decode sample qr code");
        assert_eq!("table top", std::str::from_utf8(&decoded.payload).unwrap());
    }

    #[test]
    fn can_read_qr_code_orientation_from_generated_image() {
        use quircs::*;

        let mut decoder = Quirc::default();
        let img = image::open(test_resource("table top.qr.png")).expect("could not open sample qr image");
        let img_gray = img.into_luma8();

        let codes: Vec<_> = decoder.identify(img_gray.width() as usize, img_gray.height() as usize, &img_gray).collect();

        assert_eq!(codes.len(), 1);
        let corners: [Point; 4] = codes[0].as_ref().expect("could not extract qr code from sample image").corners;
        let expected_corners: [Point; 4] = [Point { x: 4, y: 5 }, Point { x: 139, y: 3 }, Point { x: 139, y: 141 }, Point { x: 4, y: 140 }];

        assert_eq!(
            expected_corners.iter().map(|p| format!("{p:?}")).collect::<Vec<String>>(),
            corners.iter().map(|p| format!("{p:?}")).collect::<Vec<String>>()
        );
    }

    #[test]
    fn can_read_qr_code_orientation_from_rotated_generated_image() {
        use quircs::*;

        let mut decoder = Quirc::default();
        let img = image::open(test_resource("table top.rotated.qr.png")).expect("could not open sample qr image");
        let img_gray = img.into_luma8();

        let codes: Vec<_> = decoder.identify(img_gray.width() as usize, img_gray.height() as usize, &img_gray).collect();

        assert_eq!(codes.len(), 1);
        let corners: [Point; 4] = codes[0].as_ref().expect("could not extract qr code from sample image").corners;
        let expected_corners: [Point; 4] = [Point { x: 138, y: 5 }, Point { x: 138, y: 140 }, Point { x: 4, y: 141 }, Point { x: 4, y: 3 }];

        assert_eq!(
            expected_corners.iter().map(|p| format!("{p:?}")).collect::<Vec<String>>(),
            corners.iter().map(|p| format!("{p:?}")).collect::<Vec<String>>()
        );
    }
}