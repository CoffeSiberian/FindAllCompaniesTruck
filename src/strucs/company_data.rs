pub struct CompanyData {
    pub name: String,
    pub city_name: String,
    pub position: Position,
    pub parking: Vec<CompanyParking>,
}

pub struct CompanyParking {
    pub dificulty: String,
    pub position: Position,
    pub rotation: Rotation,
}

pub struct Position {
    pub x: String,
    pub y: String,
    pub z: String,
}

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
