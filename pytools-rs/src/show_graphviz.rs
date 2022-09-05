use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[derive(Copy, Clone)]
pub enum DotOutputT {
    Browser,
    SVG,
    XWindow,
}

pub trait ConvertibleToDotOutputT {
    fn to_dot_output_t(self) -> DotOutputT;
}

impl ConvertibleToDotOutputT for DotOutputT {
    fn to_dot_output_t(self) -> DotOutputT {
        self
    }
}

impl ConvertibleToDotOutputT for &str {
    fn to_dot_output_t(self) -> DotOutputT {
        match self {
            "xwindow" => DotOutputT::XWindow,
            "svg" => DotOutputT::SVG,
            "browser" => DotOutputT::Browser,
            _ => panic!("Can be one of xwindow, svg or browser"),
        }
    }
}

pub fn show_dot<T: ConvertibleToDotOutputT>(dot_code: String, output_to: T) {
    let mut fp = NamedTempFile::new().unwrap();
    writeln!(fp, "{}", dot_code).unwrap();
    let (_file, fpath) = fp.keep().unwrap();

    match output_to.to_dot_output_t() {
        DotOutputT::XWindow => {
            Command::new("dot").arg("-Tx11")
                               .arg(fpath.to_str().unwrap())
                               .output()
                               .unwrap();
        }
        DotOutputT::SVG => unimplemented!("SVG output not yet supported"),
        DotOutputT::Browser => unimplemented!("Browser output not yet supported"),
    }
}
