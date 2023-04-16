use std::fs;
use std::io::Error;
use std::path::Path;

trait PdfHtml {
    fn read_pdf<P: AsRef<Path>>(p: P) -> Result<fs::File, Error>;
    fn convert() {
        todo!()
    }
}

#[derive(Debug, Default)]
struct Unpdf;

impl PdfHtml for Unpdf {
    fn read_pdf<P: AsRef<Path>>(p: P) -> Result<fs::File, Error> {
        fs::File::open(p)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;
    #[test]
    fn read_sample_file() {
        // Read a file and copy into bytes
        const MAX_BYTES: usize = 1048576;
        let mut buf = vec![0; MAX_BYTES];
        let empty = buf.clone();
        let mut f = Unpdf::read_pdf("test_files/test1.pdf").expect("file not read");
        f.read_to_end(&mut buf).expect("buf copy failed");
        assert_ne!(buf, empty);
    }
}
