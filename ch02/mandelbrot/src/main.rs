use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

/// Determines whether `c` escapes to infinity within `limit` iterations.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

/// Parses a pair of values separated by `separator` from the string `s`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

/// Parses a complex number from a string of the form "real,imaginary".
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair::<f64>(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

/// Maps a pixel's position in the image to a point in the complex plane.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + (pixel.0 as f64 * width / bounds.0 as f64),
        im: upper_left.im - (pixel.1 as f64 * height / bounds.1 as f64),
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10x20", 'x'), Some((10, 20)));
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
    assert_eq!(parse_pair::<i32>("10,20,30", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<f64>("10,20", ','), Some((10.0, 20.0)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex("0.5,0.5"), Some(Complex { re: 0.5, im: 0.5 }));
    assert_eq!(parse_complex("1.0"), None);
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
    assert_eq!(
        pixel_to_point(
            (200, 100),
            (100, 50),
            Complex { re: -2.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: 0.0 }
    );
}
