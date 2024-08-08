use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExportData {
    pub cities: Vec<CitiesCompanyData>,
}

#[derive(Serialize, Deserialize)]
pub struct CitiesCompanyData {
    pub city_name: String,
    pub companies: Vec<CompanyDataToExport>,
}

#[derive(Serialize, Deserialize)]
pub struct CompanyDataToExport {
    pub name: String,
    pub file_name: String,
    pub position: Position,
    pub parking: Vec<CompanyParking>,
}

#[derive(Serialize, Deserialize)]
pub struct CompanyData {
    pub name: String,
    pub city: String,
    pub file_name: String,
    pub position: Position,
    pub parking: Vec<CompanyParking>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompanyParking {
    pub dificulty: String,
    pub is_hard_parking: Option<bool>,
    pub position: Position,
    pub rotation: Rotation,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Serialize, Deserialize, Clone)]
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

pub struct ValuesFlagsParking<'a> {
    pub flag_id: &'a str,
    pub is_hard_parking: bool,
}
