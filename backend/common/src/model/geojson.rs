use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub coordinates: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPoint {
    pub coordinates: Vec<Vec<f32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineString {
    pub coordinates: Vec<Vec<f32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiLineString {
    pub coordinates: Vec<Vec<Vec<f32>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Polygon {
    pub coordinates: Vec<Vec<Vec<f32>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPolygon {
    pub coordinates: Vec<Vec<Vec<Vec<f32>>>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Geometry {
    Point(Point),
    MultiPoint(MultiPoint),
    LineString(LineString),
    MultiLineString(MultiLineString),
    Polygon(Polygon),
    MultiPolygon(MultiPolygon),
    GeometryCollection(GeometryCollection),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryCollection {
    pub geometries: Vec<Geometry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    pub geometry: Option<Geometry>,
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCollection {
    pub features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GeoJSON {
    Point(Point),
    MultiPoint(MultiPoint),
    LineString(LineString),
    MultiLineString(MultiLineString),
    Polygon(Polygon),
    MultiPolygon(MultiPolygon),
    GeometryCollection(GeometryCollection),
    Feature(Feature),
    FeatureCollection(FeatureCollection),
}
