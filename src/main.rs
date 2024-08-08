mod flags_parking;
mod strucs;

use flags_parking::FLAGS_PARKING;
use rayon::prelude::*;
use serde_json::{to_string, to_string_pretty};
use std::collections::HashSet;
use std::fs::{read_dir, write, File};
use std::io::Read;
use std::sync::Mutex;
use strucs::company_data::{
    CitiesCompanyData, CompanyData, CompanyDataToExport, CompanyFindVecData, CompanyParking,
    CompanyParkingType, ExportData, Position, Rotation,
};
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

fn save_as_json(data: Vec<CitiesCompanyData>, path: &str, pretty_file: bool) -> bool {
    let export_data = ExportData { cities: data };

    let json_data = match if pretty_file {
        to_string_pretty(&export_data)
    } else {
        to_string(&export_data)
    } {
        Ok(json_data) => json_data,
        Err(_) => return false,
    };

    match write(path, json_data) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn save_as_json2(data: Vec<CompanyParkingType>, path: &str) -> bool {
    let json_data = match to_string_pretty(&data) {
        Ok(json_data) => json_data,
        Err(_) => return false,
    };

    match write(path, json_data) {
        Ok(_) => true,
        Err(_) => false,
    }
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

    let y = split_4[0]
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

    let x_string: String = split_2[0]
        .chars()
        .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
        .collect();

    let y_string: String = split_3[0]
        .chars()
        .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
        .collect();

    let z_string: String = split_3[1]
        .chars()
        .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
        .collect();

    let x = match x_string.parse::<f64>() {
        Ok(res) => res,
        Err(_) => return None,
    };

    let y = match y_string.parse::<f64>() {
        Ok(res) => res,
        Err(_) => return None,
    };

    let z = match z_string.parse::<f64>() {
        Ok(res) => res,
        Err(_) => return None,
    };

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

        let file_path = entry_data.path().to_string_lossy().to_string();
        let file_name = match entry_data.file_name().into_string() {
            Ok(file_name) => file_name,
            Err(_) => continue,
        };

        files.push(FileData {
            path: file_path,
            file_name,
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
                city: city_name,
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

    let mut node_uids_found = false;
    for item in file.iter().skip(index + 5) {
        if item.contains("}") || item.contains("]") {
            break;
        }

        if !node_uids_found && item.contains("array_u64 node_uids [") {
            node_uids_found = true;
            continue;
        }

        if node_uids_found {
            let node_id_string: String = item
                .chars()
                .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
                .collect();

            node_uids.push(node_id_string);
            continue;
        }
    }

    let mut node_flags_found = false;
    for item in file.iter().skip(index + 5 + node_uids.len() + 2) {
        if item.contains("}") || item.contains("]") {
            break;
        }

        if !node_flags_found && item.contains("array_u32 node_flags [") {
            node_flags_found = true;
            continue;
        }

        if node_flags_found {
            let node_id_string: String = item
                .chars()
                .filter(|&c| !CHARS_TO_REMOVE_BASIC.contains(c))
                .collect();

            node_flags.push(node_id_string);
            continue;
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
    for (i, item) in node_uids.iter().enumerate() {
        let (position, rotation): (Position, Rotation) =
            match get_node_item_data(file, item.clone(), index_node_uid) {
                Some(res) => res,
                None => continue,
            };
        let node_flags_num = node_flags[i].clone();
        let is_hard_parking = FLAGS_PARKING.iter().find(|&x| x.flag_id == node_flags_num);

        let company_parking_data: CompanyParking = CompanyParking {
            dificulty: node_flags_num,
            is_hard_parking: match is_hard_parking {
                Some(res) => Some(res.is_hard_parking),
                None => None,
            },
            position,
            rotation,
        };

        parking_data.push(company_parking_data);
    }

    if parking_data.is_empty() {
        return None;
    }

    return Some(parking_data);
}

fn get_file_company_data(file: &Vec<String>, file_name: &String) -> Option<Vec<CompanyData>> {
    let (companies, nodes_index): (Vec<CompanyFindVecData>, usize) = match get_file_companies(file)
    {
        Some(companies) => companies,
        None => return None,
    };

    let mut companies_data: Vec<CompanyData> = Vec::new();
    for item in &companies {
        let parking_data: Vec<CompanyParking> =
            match get_parking_data_company(file, item.index_company, nodes_index) {
                Some(res) => res,
                None => continue,
            };

        let (company_position, _company_rotation) =
            match get_node_item_data(file, item.node_uid.clone(), nodes_index) {
                Some(res) => res,
                None => continue,
            };

        let company_data: CompanyData = CompanyData {
            name: item.name.clone(),
            city: item.city.clone(),
            file_name: file_name.clone(),
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

fn get_all_company_map(file_data: &Vec<FileData>) -> Option<Vec<CitiesCompanyData>> {
    let all_companies: Mutex<Vec<CitiesCompanyData>> = Mutex::new(Vec::new());
    let cities_found: Mutex<HashSet<String>> = Mutex::new(HashSet::new());

    file_data.par_iter().for_each(|item| {
        let file = match read_file_text_vec(&item.path) {
            Some(res) => res,
            None => return,
        };

        let company = match get_file_company_data(&file, &item.file_name) {
            Some(res) => res,
            None => return,
        };

        let mut all_companies_data = match all_companies.lock() {
            Ok(res) => res,
            Err(_) => return,
        };
        let mut cities_found_data = match cities_found.lock() {
            Ok(res) => res,
            Err(_) => return,
        };

        for item in company {
            let company_export = CompanyDataToExport {
                name: item.name.clone(),
                file_name: item.file_name.clone(),
                position: item.position.clone(),
                parking: item.parking.clone(),
            };
            if !cities_found_data.contains(item.city.as_str()) {
                cities_found_data.insert(item.city.clone());
                all_companies_data.push(CitiesCompanyData {
                    city_name: item.city.clone(),
                    companies: vec![company_export],
                });
            } else {
                for item2 in all_companies_data.iter_mut() {
                    if item2.city_name == item.city {
                        item2.companies.push(company_export);
                        break;
                    }
                }
            }
        }
    });

    let all_companies_check: Vec<CitiesCompanyData> = match all_companies.into_inner() {
        Ok(res) => res,
        Err(_) => return None,
    };

    if all_companies_check.is_empty() {
        return None;
    }

    return Some(all_companies_check);
}

#[allow(dead_code)]
fn get_any_flags_id_not_repeated(companies: &Vec<CompanyData>, path: &str) -> bool {
    let mut flags_set: HashSet<String> = HashSet::new();
    let mut flags_vec: Vec<CompanyParkingType> = Vec::new();

    for item in companies {
        for item2 in item.parking.iter() {
            let res = flags_set.insert(item2.dificulty.clone());

            if res {
                let company_parking_type = CompanyParkingType {
                    dificulty: match item2.dificulty.parse::<u16>() {
                        Ok(res) => res,
                        Err(_) => continue,
                    },
                    file_name: item.file_name.clone(),
                };

                flags_vec.push(company_parking_type);
            }
        }
    }

    if flags_vec.is_empty() {
        return false;
    }

    save_as_json2(flags_vec, path);
    return true;
}

fn main() {
    let (files, total_files): (Vec<FileData>, usize) = match list_files("path") {
        Some(dir_content) => dir_content,
        None => return,
    };

    println!("Total files: {}", total_files);
    let companies: Vec<CitiesCompanyData> = match get_all_company_map(&files) {
        Some(res) => res,
        None => {
            println!("No companies found");
            return;
        }
    };
    println!("Total cities: {}", companies.len());

    //get_any_flags_id_not_repeated(&companies, "path");
    save_as_json(companies, "path", true);
    return;
}
