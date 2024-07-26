mod strucs;

use std::fs::{read_dir, File};
use std::io::Read;
use strucs::company_data::{CompanyData, CompanyFindVecData, CompanyParking, Position, Rotation};
use strucs::file_data::FileData;

const CHARS_TO_REMOVE_BASIC: &str = " \"";

fn read_file(path: &str) -> Option<String> {
    let file_open = File::open(path);

    let mut file = match file_open {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut buffer: String = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(_) => return None,
    };

    return Some(buffer);
}

fn read_file_text_vec(path: &str) -> Option<Vec<String>> {
    let file = match read_file(path) {
        Some(file) => file,
        None => return None,
    };

    return Some(file.lines().map(|s| s.to_string()).collect());
}

fn get_string_rotation(values: String) -> Option<Rotation> {
    let split_1: Vec<&str> = values.split("w:").collect::<Vec<&str>>();
    let split_2: Vec<&str> = split_1[1].split("x:").collect::<Vec<&str>>();
    let split_3: Vec<&str> = split_2[1].split("y:").collect::<Vec<&str>>();
    let split_4: Vec<&str> = split_3[1].split("z:").collect::<Vec<&str>>();

    let w = split_2[0]
        .chars()
        .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
        .collect();

    let x = split_3[0]
        .chars()
        .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
        .collect();

    let y = split_3[1]
        .chars()
        .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
        .collect();

    let z = split_4[1]
        .chars()
        .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
        .collect();

    return Some(Rotation { w, x, y, z });
}

fn get_string_position(values: String) -> Option<Position> {
    let split_1: Vec<&str> = values.split("x:").collect::<Vec<&str>>();
    let split_2: Vec<&str> = split_1[1].split("y:").collect::<Vec<&str>>();
    let split_3: Vec<&str> = split_2[1].split("z:").collect::<Vec<&str>>();

    let x = split_2[0]
        .chars()
        .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
        .collect();

    let y = split_3[0]
        .chars()
        .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
        .collect();

    let z = split_3[1]
        .chars()
        .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
        .collect();

    return Some(Position { x, y, z });
}

fn list_files(path: &str) -> Option<(Vec<FileData>, usize)> {
    let entries = match read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return None,
    };

    let mut files: Vec<FileData> = Vec::new();

    for entry in entries {
        let entry_data = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let file_extension = match entry_data.path().extension() {
            Some(extension) => extension.to_string_lossy().to_string(),
            None => continue,
        };

        if file_extension != "base" {
            continue;
        }

        let file_name = match entry_data.file_name().into_string() {
            Ok(file_name) => file_name,
            Err(_) => continue,
        };
        let file_path = entry_data.path().to_string_lossy().to_string();

        files.push(FileData {
            name: file_name,
            path: file_path,
            extension: file_extension,
        });
    }

    if files.is_empty() {
        return None;
    }

    let total_files = files.len();

    return Some((files, total_files));
}

fn get_file_companies(file: &Vec<String>) -> Option<(Vec<CompanyFindVecData>, usize)> {
    let mut companies: Vec<CompanyFindVecData> = Vec::new();
    let mut nodes_index: usize = 0;

    for (i, item) in file.iter().enumerate() {
        if item.contains("token company_name:") {
            let split_str_company: Vec<&str> = item.split(":").collect();
            let split_str_city: Vec<&str> = file[i + 1].split(":").collect();
            let split_str_node_uid: Vec<&str> = file[i + 3].split(":").collect();

            let company_name = split_str_company[1]
                .chars()
                .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
                .collect();

            let city_name = split_str_city[1]
                .chars()
                .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
                .collect();

            let node_uid = split_str_node_uid[1]
                .chars()
                .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
                .collect();

            companies.push(CompanyFindVecData {
                name: company_name,
                city_name: city_name,
                node_uid: node_uid,
                index_company: i,
            });
        }
        if item.contains("array_struct nodes [") {
            nodes_index = i;
            break;
        }
    }

    if companies.is_empty() {
        return None;
    }

    return Some((companies, nodes_index));
}

fn get_parking_uids(file: &Vec<String>, index: usize) -> Option<(Vec<String>, Vec<String>)> {
    let mut node_uids: Vec<String> = Vec::new();
    let mut node_flags: Vec<String> = Vec::new();

    let mut node_flags_found = false;
    for item in file.iter().skip(index + 5) {
        if item.contains("}") {
            break;
        }

        if item.contains("array_u32 node_flags [") {
            node_flags_found = true;
            continue;
        }

        let node_id_string: String = item
            .chars()
            .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
            .collect();

        if node_flags_found {
            node_flags.push(node_id_string);
            continue;
        } else {
            node_uids.push(node_id_string);
        }
    }

    if node_uids.is_empty() || node_flags.is_empty() {
        return None;
    }

    return Some((node_uids, node_flags));
}

fn get_node_item_data(
    file: &Vec<String>,
    uid: String,
    index_node_uid: usize,
) -> Option<(Position, Rotation)> {
    for (i, item) in file.iter().enumerate().skip(index_node_uid) {
        if item.contains("u64 uid: ") {
            let split_str_uid: Vec<&str> = item.split(":").collect();

            let node_uid: String = split_str_uid[1]
                .chars()
                .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
                .collect();

            if node_uid != uid {
                continue;
            }

            let position_values = match get_string_position(file[i + 1].clone()) {
                Some(res) => res,
                None => continue,
            };
            let rotation_values = match get_string_rotation(file[i + 2].clone()) {
                Some(res) => res,
                None => continue,
            };

            return Some((position_values, rotation_values));
        }
    }

    return None;
}

fn get_parking_data_company(
    file: &Vec<String>,
    index_company: usize,
    index_node_uid: usize,
) -> Option<Vec<CompanyParking>> {
    let (node_uids, node_flags): (Vec<String>, Vec<String>) =
        match get_parking_uids(file, index_company) {
            Some(res) => res,
            None => return None,
        };

    let mut parking_data: Vec<CompanyParking> = Vec::new();
    for (_i, item) in node_uids.iter().enumerate() {
        let (position, rotation): (Position, Rotation) =
            match get_node_item_data(file, item.clone(), index_node_uid) {
                Some(res) => res,
                None => continue,
            };

        parking_data.push(CompanyParking {
            dificulty: "test".to_string(),
            position: position,
            rotation: rotation,
        });
    }

    if parking_data.is_empty() {
        return None;
    }

    return Some(parking_data);
}

fn get_file_company_data(file: &Vec<String>) -> Option<Vec<CompanyData>> {
    let (companies, nodes_index): (Vec<CompanyFindVecData>, usize) = match get_file_companies(file)
    {
        Some(companies) => companies,
        None => return None,
    };

    let mut companies_data: Vec<CompanyData> = Vec::new();
    for item in companies {
        let parking_data: Vec<CompanyParking> =
            match get_parking_data_company(file, item.index_company, nodes_index) {
                Some(res) => res,
                None => continue,
            };

        let (company_position, _company_rotation) =
            match get_node_item_data(file, item.node_uid, nodes_index) {
                Some(res) => res,
                None => continue,
            };

        let company_data = CompanyData {
            name: item.name,
            city_name: item.city_name,
            position: company_position,
            parking: parking_data,
        };

        companies_data.push(company_data);
    }

    if companies_data.is_empty() {
        return None;
    }

    return Some(companies_data);
}

fn get_all_company_map(file_data: &Vec<FileData>) -> Option<Vec<CompanyData>> {
    let mut all_companies: Vec<CompanyData> = Vec::new();

    for item in file_data {
        let file = match read_file_text_vec(&item.path) {
            Some(res) => res,
            None => continue,
        };

        match get_file_company_data(&file) {
            Some(companies) => {
                all_companies.extend(companies);
            }
            None => continue,
        }
    }

    if all_companies.is_empty() {
        return None;
    }

    return Some(all_companies);
}

fn main() {
    let dir_content = match list_files("path") {
        Some(dir_content) => dir_content,
        None => {
            return;
        }
    };

    let (files, total_files) = dir_content;

    println!("Total files: {}", total_files);
    get_all_company_map(&files);
}
