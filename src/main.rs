use std::collections::{HashMap};


//CONSTANTES
const PATH: &str = "src/dataset.csv";


fn main() {
    let mut df: HashMap<String, Vec<String>> = HashMap::new();


    //LEEMOS EL CSV
    let mut headers: Vec<String> = Vec::new();

    read_csv(&mut df, PATH, &mut headers);
    // println!("{:?}", headers);
    // println!("{:?}", df);

    //OBTENEMOS LOS DIFERENTES VALORES DE CADA VECTOR
    let unique_values = get_unique_values(&df);

    //-------------------REGLAS DE ASOCIACIÓN-------------------//
    //COMBINATORIA DE LOS HEADERS
    let mut result_headers: Vec<Vec<String>> = Vec::new();
    combine(&headers, 0, &mut result_headers);
    println!("Combinatoria de los Headers: \n{:?}\n", result_headers);

    //COMBINATORIA DE LOS VALORES UNICOS DE LA COMBINATORIA DE LOS HEADERS USEMOS UNA FUNCION
    let mut result_values: Vec<Vec<String>> = Vec::new();

    println!("Valores únicos: \n{:?}\n\n", unique_values);


    //PARA CADA HEADER EN LA COMBINATORIA DE LOS HEADERS
    for headers in result_headers {
        //COMBINATORIA DE LOS VALORES UNICOS DE CADA HEADER
        let mut values: Vec<Vec<String>> = Vec::new();
        //OBTENEMOS LOS VALORES UNICOS DE CADA HEADER
        let header_one_values = unique_values.get(&headers[0]).unwrap();
        let header_two_values = unique_values.get(&headers[1]).unwrap();

        //OBTENEMOS LAS COMBINACIONES DE LOS VALORES UNICOS DE CADA HEADER
        values = values_combinations(header_one_values, header_two_values);
        println!("Combinatoria de los valores únicos de {:?} y {:?}: \n{:?}\n", headers[0], headers[1], values);

        

        // combine(&headers, 0, &mut values);
        // println!("{:?}\n", values);
    }

    

    
}
//FUNCIONES
fn values_combinations(header_one_values: &Vec<String>, header_two_values: &Vec<String>) -> Vec<Vec<String>> {
    let mut values: Vec<Vec<String>> = Vec::new();
    for value_one in header_one_values {
        for value_two in header_two_values {
            values.push(vec![value_one.clone(), value_two.clone()]);
        }
    }
    values
}

fn combine(elements: &Vec<String>, start: usize, result: &mut Vec<Vec<String>>) {
    let n = elements.len();
    for i in start..n {
        for j in (i+1)..n {
            result.push(vec![elements[i].clone(), elements[j].clone()]);
        }
    }
}

//FUNCIONES
fn get_unique_values(df: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut unique_values: HashMap<String, Vec<String>> = HashMap::new();

    for (header, values) in df.iter() {
        let mut unique_values_vec: Vec<String> = Vec::new();
        for value in values {
            if !unique_values_vec.contains(value) {
                unique_values_vec.push(value.to_string());
            }
        }
        unique_values.insert(header.to_string(), unique_values_vec);
    }

    unique_values
}

fn read_csv(df: &mut HashMap<String, Vec<String>>, path: &str, headers: &mut Vec<String>) {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .unwrap();

    // Obtenemos los headers:
    let headers_string_record = rdr.headers().unwrap().clone();
    for header in headers_string_record.iter() {
        headers.push(header.to_string());
    }

    let mut records = vec![];

    for result in rdr.records() {
        let record = result.unwrap();
        records.push(record);
    }

    for record in &records {
        for (j, col) in record.iter().enumerate() {
            let header = headers[j].to_string();
            df.entry(header).or_insert_with(Vec::new).push(col.to_string());
        }
    }
}