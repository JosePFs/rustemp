use std::fmt::{Display, Formatter};

use crate::domain::municipality::Municipality;

#[derive(Debug, Clone)]
pub struct Name(pub String);

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Name {
    pub fn from_str(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_lowercase(&self) -> String {
        self.0.to_lowercase()
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct GeometryCoordinates {
    pub longitude: f64,
    pub latitude: f64,
}

impl Default for GeometryCoordinates {
    fn default() -> Self {
        Self {
            longitude: 0.0,
            latitude: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GeometryType {
    Point,
    LineString,
    Polygon,
}

impl From<&str> for GeometryType {
    fn from(value: &str) -> Self {
        match value {
            "Point" => GeometryType::Point,
            "LineString" => GeometryType::LineString,
            "Polygon" => GeometryType::Polygon,
            _ => GeometryType::Point,
        }
    }
}

impl Default for GeometryType {
    fn default() -> Self {
        GeometryType::Point
    }
}

#[derive(Debug, Clone)]
pub struct Geometry {
    pub r#type: GeometryType,
    pub coordinates: GeometryCoordinates,
}

impl Geometry {
    pub fn new(r#type: GeometryType, coordinates: GeometryCoordinates) -> Self {
        Self {
            r#type,
            coordinates,
        }
    }
}

impl Default for Geometry {
    fn default() -> Self {
        Self {
            r#type: GeometryType::default(),
            coordinates: GeometryCoordinates::default(),
        }
    }
}

impl GeometryCoordinates {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            longitude,
            latitude,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Place {
    pub name: Name,
    pub municipality: Municipality,
}

impl Place {
    pub fn new(name: Name, municipality: Municipality) -> Self {
        Self { name, municipality }
    }
}

impl Display for Place {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct PlaceType(pub String);

impl From<&str> for PlaceType {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Display for PlaceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub id: String,
    pub place: Place,
    pub geometry: Geometry,
}

impl Location {
    pub fn new(id: String, place: Place, geometry: Geometry) -> Self {
        Self {
            id,
            place,
            geometry,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
