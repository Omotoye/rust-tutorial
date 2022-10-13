// The formatting functionality is implemented via traits, and there is one
// trait for each argument type. The most comman formatting trait is Display,
// which handles cases where the argument type is left unspecified: {} for instance.

use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,

    // Latitude
    lat: f32,

    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string
    // into it
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into the buffer (the first argument)
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

/**** Activity ****** */
// Add an implementation of the fmt::Display trait for the Color
// struct above so that the output displays as:

// RGB (128, 255, 90) Ox80FF5A
// RGB (0, 3, 254) 0x0003FE
// RGB (128, 255, 90) Ox000000

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // let red_0 = if self.red < 10 { "0" } else { "" };
        // let green_0 = if self.green < 10 { "0" } else { "" };
        // let blue_0 = if self.blue < 10 { "0" } else { "" };

        // write!(
        // f,
        // "RGB ({0}, {1}, {2}) 0x{3}{0:x}{4}{1:x}{5}{2:x}",
        // self.red, self.green, self.blue, red_0, green_0, blue_0,
        // )

        /***** The Recommended way *******/
        write!(
            f,
            "RGB ({0}, {1}, {2}) 0x{0:0>2x}{1:0>2x}{2:0>2x}",
            self.red, self.green, self.blue,
        )
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vacouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    println!();
    for color in [
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
    ]
    .iter()
    {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display
        println!("{}", *color);
        // println!("{:?}", *color);
    }
}
