use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CompanyData {
    pub name: String,
    pub city_name: String,
    pub position: Position,
    pub parking: Vec<CompanyParking>,
}

#[derive(Serialize, Deserialize)]
pub struct CompanyParking {
    pub dificulty: String,
    pub position: Position,
    pub rotation: Rotation,
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub x: String,
    pub y: String,
    pub z: String,
}

#[derive(Serialize, Deserialize)]
pub struct Rotation {
    pub w: String,
    pub x: String,
    pub y: String,
    pub z: String,
}
pub struct CompanyFindVecData {
    pub name: String,
    pub city_name: String,
    pub node_uid: String,
    pub index_company: usize,
}
