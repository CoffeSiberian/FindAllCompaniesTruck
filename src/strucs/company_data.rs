use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CitiesCompanyData {
    pub city_name: String,
    pub companies: Vec<CompanyData>,
}

#[derive(Serialize, Deserialize)]
pub struct CompanyData {
    pub name: String,
    pub city: String,
    pub file_name: String,
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
    pub city: String,
    pub node_uid: String,
    pub index_company: usize,
}

#[derive(Serialize, Deserialize)]
pub struct CompanyParkingType {
    pub dificulty: u16,
    pub file_name: String,
}

#[allow(dead_code)]
pub struct ValuesFlagsParking<'a> {
    pub flag_id: &'a str,
    pub is_hard_parking: bool,
}
