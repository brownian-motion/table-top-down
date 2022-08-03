
#[cfg(test)]
mod test {
    #[test]
    fn can_load_test_files(){
        //https://stackoverflow.com/questions/30003921/how-can-i-locate-resources-for-testing-with-cargo
        use std::path::PathBuf;
        use std::fs::File;
        use std::io::prelude::*;

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test/read_test.txt");

        let mut f = File::open(path).expect("could not open test resource file");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("could not read from test resource file");

        assert_eq!("hello world!", &contents);
    }
}