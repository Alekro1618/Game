
use crate::error::error::{Error, ErrorKind};

fn load_file(path : &'static str, on_loaded : impl Fn(Result<Vec<u8>, Error>) + 'static) {
    macroquad::miniquad::fs::load_file(path, move |result| {
        on_loaded(
            result.map_err(|_| Error::new(ErrorKind::IO, format!("Failed to load asset at path {}", path)))
        );
    });
}