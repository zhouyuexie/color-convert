use handles::hex;
use std::fmt;
use std::error::Error as StdError;

// color define
#[derive(Debug)]
pub struct Color<'a> {
	pub mode: ColorMode<'a>,
	pub to_upper: bool,
	pub to_android: bool,
	pub to_alpha: bool
}
impl<'a> Color<'a> {
	// new one Color instance by all setting
	pub fn new(mode: &'a str, to_upper: bool, to_android: bool, to_alpha: bool) -> Self {
		Color {
			mode: ColorMode::new(mode),
			to_upper,
			to_android,
			to_alpha
		}
	}
	// new one color instance by default config
	pub fn init(mode: &'a str) -> Self {
		Color {
			mode: ColorMode::new(mode),
			to_upper: false,
			to_android: false,
			to_alpha: false
		}
	}

	// config Color config [to_upper, to_android, to_alpha]
	pub fn upper(&mut self, to_upper: bool) -> &Self {
		self.to_upper = to_upper;
		self
	}
	pub fn android(&mut self, to_android: bool) -> &Self {
		self.to_android = to_android;
		self
	}
	pub fn alpha(&mut self, to_alpha: bool) -> &Self {
		self.to_alpha = to_alpha;
		self
	}
	pub fn to_str(&self) -> &str {
		match self.mode {
			ColorMode::HEX(color) => color,
			ColorMode::RGB(color) => color,
			ColorMode::RGBA(color) => color,
			ColorMode::HSL(color) => color,
			ColorMode::HSLA(color) => color,
			ColorMode::HSV(color) => color,
			ColorMode::CMYK(color) => color,
		}
	}

	pub fn to_rgb(&self) -> Result<String, Error> {
		match self.mode {
			ColorMode::HEX(_) => hex::hex2rgb(self),
			_ => Err(Error::NotMatch)
		}
	}

	pub fn to_hex(&self) -> Result<String, Error> {
		match self.mode {
			ColorMode::HEX(_) => hex::hex2hex(self),
			_ => Err(Error::NotMatch)
		}
	}
}

#[derive(Debug)]
// #[derive(Eq)]
pub enum ColorMode<'a> {
	RGBA(&'a str),
	RGB(&'a str),
	HSLA(&'a str),
	HSL(&'a str),
	CMYK(&'a str),
	HSV(&'a str),
	HEX(&'a str)
}

impl<'a> ColorMode<'a> {
	pub fn new(color: &'a str) -> ColorMode {
		let upper_color = color.to_ascii_uppercase();

		if upper_color.contains("RGBA") {
			ColorMode::RGBA(color)
		} else if upper_color.contains("RGB") {
			ColorMode::RGB(color)
		} else if upper_color.contains("HSLA") {
			ColorMode::HSLA(color)
		} else if upper_color.contains("HSL") {
			ColorMode::HSL(color)
		} else if upper_color.contains("CMYK") {
			ColorMode::CMYK(color)
		} else if upper_color.contains("HSV") {
			ColorMode::HSV(color)
		} else {
			ColorMode::HEX(color)
		}
	}
}

impl<'a> PartialEq for ColorMode<'a> {
	fn eq(&self, other: &ColorMode) -> bool {
		match (self, other) {
			(ColorMode::RGBA(i), ColorMode::RGBA(j)) => i == j,
			(ColorMode::RGB(i), ColorMode::RGB(j)) => i == j,
			(ColorMode::HSLA(i), ColorMode::HSLA(j)) => i == j,
			(ColorMode::HSL(i), ColorMode::HSL(j)) => i == j,
			(ColorMode::CMYK(i), ColorMode::CMYK(j)) => i == j,
			(ColorMode::HSV(i), ColorMode::HSV(j)) => i == j,
			(ColorMode::HEX(i), ColorMode::HEX(j)) => i == j,
			_ => false
		}
	}
}

#[derive(Debug)]
pub enum Error {
	Format,
	NotMatch
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::Format => write!(f, "Format Error"),
			Error::NotMatch => write!(f, "NotMatch Color")
		}
	}
}

impl StdError for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Format => "[color-convert] Color value is illegal.",
			Error::NotMatch => "[color-convert] No match color value.",
		}
	}
}