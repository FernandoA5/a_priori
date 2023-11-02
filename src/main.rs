use std::{fs::File};
use std::collections::HashMap;

fn main() {
    //Leemos el CSV:
    let (matrix, headers) = read_csv("titanic.csv");
    //Imprimimos la matrix:

    // println!("Imprimimos la matrix:");
    // imprimir_matrix(&matrix);

    //Obtenemos los valores diferentes de la columna 1:
    let map: HashMap<String, Vec<String>> = get_values(&matrix, &headers);
    println!("Imprimimos el mapa:");
    println!("{:?}", map);
    let mut permutations: Vec<String> = generate_combinations(&map, 0);
    println!("Imprimimos las permutaciones:");
    println!("{:?}", permutations);

    //Creamos las permutaciones de los valores del mapa:
    let mut contador: i32 = 0;


}

fn generate_combinations(map: &HashMap<String, Vec<String>>, index: usize) -> Vec<String> {
    if index >= map.len() {
        return Vec::new();
    }

    let mut combinations = Vec::new();
    let keys: Vec<&String> = map.keys().collect();
    let current_key = keys[index];
    let current_values = &map[current_key];

    for i in 0..current_values.len() {
        for j in i + 1..current_values.len() {
            let x = current_values[i].clone();
            let y = current_values[j].clone();
            combinations.push(format!("IF {} THEN {}", x, y));
        }
    }

    let mut next_combinations = generate_combinations(map, index + 1);
    combinations.append(&mut next_combinations);

    combinations
}



fn get_values(matrix: &Vec<Vec<String>>, headers: &Vec<String>) -> HashMap<String, Vec<String>> {
    //Variables:
    //Hacemos un key-value con: key: Header, value: Vec<String>
    let mut map: HashMap<String, Vec<String>> = std::collections::HashMap::new();
    //Agregamos las KEY iterando sobre los headers:
    for header in headers.iter() {
        map.insert(header.to_string(), Vec::new());
    }
    println!("Mapa: {:?}", map);
    //Recorremos primero por columnas y luego por filas
    for col in 0..matrix[0].len()
    {
        for fila in 0..matrix.len()
        {
            if map[&headers[col]].contains(&matrix[fila][col]) == false && matrix[fila][col] != "" {
                map.get_mut(&headers[col]).unwrap().push(matrix[fila][col].to_string());   
            }
        }
        println!("");
    }
    map
}

fn read_csv(path: &str) -> (Vec<Vec<String>>, Vec<String>){
    let mut matrix: Vec<Vec<String>> = Vec::new();
    // Abrimos el archivo
    let file = File::open(path).expect("No se pudo abrir el archivo");

    // Creamos un lector de CSV:
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);


    let headers = rdr.headers().unwrap().clone();
    let mut headers_list: Vec<String> = Vec::new();
    
    for header in headers.iter() {
        headers_list.push(header.to_string());
    }

    // Iteramos sobre los registros
    for result in rdr.records() {
        // Desempaquetamos el resultado
        let record = result.expect("No se pudo leer el registro");
        
        //Son 3 columnas: Survived, Pclass, Embarked (Pero nosotros lo sabemos, el programa no) So: iteramos sobre las columnas
        let mut lista: Vec<String> = Vec::new();
        for column in record.iter() {
            lista.push(column.to_string());
        }

        matrix.push(lista.clone());
    }

    (matrix, headers_list)
}

fn imprimir_matrix(matrix: &Vec<Vec<String>>){
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
}