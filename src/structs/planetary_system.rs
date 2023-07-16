use eframe::epaint::Color32;
use serde::Deserialize;

#[derive(Debug)]
pub struct PlanetarySystem {
	pub name: String,
	pub objects: Vec<PlanetaryObject>,
}

impl PlanetarySystem {
	pub fn new(objects: Vec<PlanetaryObject>, name: String) -> Self {
		Self { name, objects }
	}
}

#[derive(Debug)]
pub struct PlanetaryObject {
	pub name: String,
	pub distance_km: f64,
	pub distance_au: f64,
	pub eccentricity: f64,
	pub period_years: f64,
	pub group: String,
	pub colour: Color32,
}

impl PlanetaryObject {
	pub fn from_raw(planetary_object_raw: PlanetaryObjectRaw) -> Self {
		Self {
			name: planetary_object_raw.name,
			distance_km: planetary_object_raw.distance_km,
			distance_au: planetary_object_raw.distance_km / crate::constants::ASTRONOMICAL_UNIT_KM,
			eccentricity: planetary_object_raw.eccentricity,
			period_years: planetary_object_raw.period_years,
			group: planetary_object_raw.group,
			colour: parse_colour(planetary_object_raw.colour, Color32::WHITE),
		}
	}
}

#[derive(Deserialize)]
pub struct PlanetaryObjectRaw {
	name: String,
	distance_km: f64,
	eccentricity: f64,
	period_years: f64,
	group: String,
	colour: Option<String>,
}

fn parse_colour(col: Option<String>, default_colour: Color32) -> Color32 {
	if let Some(colour_string) = col {
		if let Ok(mut col_raw) = i64::from_str_radix(&colour_string, 16) {
			let a = col_raw % 256;
			col_raw /= 256; // a < 256, so there is no need to subtract it before division as it can only create a decimal part which is dropped in integer division
			let b = col_raw % 256;
			col_raw /= 256;
			let g = col_raw % 256;
			col_raw /= 256;
			let r = col_raw;
			Color32::from_rgba_premultiplied(r as u8, g as u8, b as u8, a as u8)
		} else {
			default_colour
		}
	} else {
		default_colour
	}
}
