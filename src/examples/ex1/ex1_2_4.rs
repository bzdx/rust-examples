// 
use std::fmt;

#[allow(dead_code)]
pub fn run() {
  struct City {
    name: &'static str,
    // 维度
    lat: f32,
    // 精度
    lon: f32,
  }

  impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
      let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
      write!(f, "{city_name}: {lat:.3}°{lat_c} {lon:.3}°{lon_c}",
      city_name = self.name, lat = self.lat.abs(), lat_c = lat_c, lon = self.lon.abs(), lon_c = lon_c)
    }
  }

  #[derive(Debug)]
  struct Color {
    red: u8,
    green: u8,
    blue: u8,
  }

  for color in [
    Color { red: 125, green: 32, blue: 120 },
    Color { red: 34, green: 55, blue: 67 },
    Color { red: 11, green: 55, blue: 33 },
    Color { red: 45, green: 52, blue: 25 },
  ].iter() {
    println!("{:#?}", *color);
  }

  for city in [
    City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
    City { name: "Oslo", lat: 59.95, lon: 10.75 },
    City { name: "Vancouver", lat: 49.25, lon: -123.1 },
  ].iter() {
    println!("{}", *city);
  }
}