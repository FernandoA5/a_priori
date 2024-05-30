use std::collections::HashMap;


//CONSTANTES
const PATH: &str = "src/dataset.csv";
const MIN_SUPPORT: f64 = 0.5;

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


    println!("Valores únicos: \n{:?}\n\n", unique_values);

    let mut reglas_asociacion: Vec<String> = Vec::new();


    //PARA CADA HEADER EN LA COMBINATORIA DE LOS HEADERS
    for headers in result_headers {
        //COMBINATORIA DE LOS VALORES UNICOS DE CADA HEADER
        //OBTENEMOS LOS VALORES UNICOS DE CADA HEADER
        let header_one_values = unique_values.get(&headers[0]).unwrap();
        let header_two_values = unique_values.get(&headers[1]).unwrap();

        //OBTENEMOS LAS COMBINACIONES DE LOS VALORES UNICOS DE CADA HEADER
        let values: Vec<Vec<String>> = values_combinations(header_one_values, header_two_values);
        println!("Combinatoria de los valores únicos de NX:{:?} y NY:{:?}:", headers[0], headers[1]);
        
        for value in &values {
            
            //OCURRENCIAS DE VALUE EN LA COLUMNA DE HEADER[0] Y HEADER[1] EN EL DATAFRAME
            let ocurrences_header_one = filter_occurrences(&df, &headers[0], &value[0]);
            let ocurrences_header_two = filter_occurrences(&df, &headers[1], &value[1]);
            let count_header_one = ocurrences_header_one.get(&headers[0]).unwrap().len();
            let count_header_two = ocurrences_header_two.get(&headers[1]).unwrap().len();
            let total_data = df.get(&headers[0]).unwrap().len();
            
            // CANTIDAD DE OCURRENCIAS DE LA COMBINACIÓN DE VALUE EN EL DATAFRAME (NX^Y)
            let ocurrences_combined = filter_combined_occurrences(&df, &df, &headers[0], &value[0], &headers[1], &value[1]);
            let nx_y = ocurrences_combined.get(&headers[0]).unwrap().len();

            //SOPORTE DE LA COMBINACIÓN DE VALUE
            let support = nx_y as f64 / total_data as f64;

            //CONFIANZA DE LA COMBINACIÓN DE VALUE
            let confidence = nx_y as f64 / count_header_one as f64;

            //CORRELACIÓN DE LA COMBINACIÓN DE VALUE
            let lift = (nx_y as f64 * total_data as f64) / (count_header_one as f64 * count_header_two as f64);

            // println!("{:?}",ocurrences_header_one);
            

            // println!("NX={}: {count_header_one}, NY={}: {count_header_two}, NX^Y:{nx_y}, S:{support}, C: {confidence}, L: {lift}", value[0], value[1]);

            //CONSERVAR LAS REGLAS DE ASOCIACIÓN QUE CUMPLAN CON EL SOPORTE MÍNIMO
            if support >= MIN_SUPPORT {
                let regla = format!("{}:{}-{}:{}", headers[0], value[0],headers[1], value[1]);
                // println!("Regla: {regla}");
                reglas_asociacion.push(regla);
            }
        }
    }

    println!("\n\n");
    //REGLAS DE SEGUNDO, TERCER, etc, GRADO
    println!("{:?}", reglas_asociacion);
    //AQUÍ EMPEZAMOS EL CÓDIGO QUE ITERARÁ SOBRE EL NESIMO GRADO DE REGLAS DE ASOCIACIÓN
    let num_headers = headers.len();
    for i in 2..num_headers {
        let mut new_reglas_asociacion: Vec<String> = Vec::new();
        //EXTRAEMOS LOS VALORES Y HEADERS DE LAS REGLAS DE ASOCIACIÓN ANTERIORES
        get_rules_info(&reglas_asociacion);
    }


    
    
}
//FUNCIONES

fn get_rules_info(rules: &Vec<String>) -> Vec<Vec<String>> {
    let mut rules_info: Vec<Vec<String>> = Vec::new();
    for rule in rules {
        let rule_info: Vec<String> = rule.split("-").map(|s| s.to_string()).collect();
        rules_info.push(rule_info);
    }
    rules_info
}

// FUNCION QUE DEVUELVE UN DATAFRAME CON LAS OCURRENCIAS DE UNA COMBINACIÓN DE VALORES EN DOS DATAFRAMES
fn filter_combined_occurrences(
    df1: &HashMap<String, Vec<String>>,
    df2: &HashMap<String, Vec<String>>,
    header1: &String,
    value1: &String,
    header2: &String,
    value2: &String
) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    
    // Inicializa las columnas en el nuevo dataframe
    result.insert(header1.clone(), Vec::new());
    result.insert(header2.clone(), Vec::new());

    if let (Some(values1), Some(values2)) = (df1.get(header1), df2.get(header2)) {
        for (val1, val2) in values1.iter().zip(values2) {
            if val1 == value1 && val2 == value2 {
                result.get_mut(header1).unwrap().push(val1.clone());
                result.get_mut(header2).unwrap().push(val2.clone());
            }
        }
    }
    
    result
}


//FUNCION QUE FILTRA LAS OCURRENCIAS DE UN VALOR EN UNA COLUMNA
fn filter_occurrences(df: &HashMap<String, Vec<String>>, header: &String, value: &String) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    
    // Inicializa la columna en el nuevo dataframe
    result.insert(header.clone(), Vec::new());

    if let Some(values) = df.get(header) {
        for val in values {
            if val == value {
                result.get_mut(header).unwrap().push(val.clone());
            }
        }
    }
    
    result
}


//FUNCION QUE OBTIENE LAS COMBINACIONES DE LOS VALORES UNICOS DE DOS HEADERS
fn values_combinations(header_one_values: &Vec<String>, header_two_values: &Vec<String>) -> Vec<Vec<String>> {
    let mut values: Vec<Vec<String>> = Vec::new();
    for value_one in header_one_values {
        for value_two in header_two_values {
            values.push(vec![value_one.clone(), value_two.clone()]);
        }
    }
    values
}

//FUNCION QUE OBTIENE LA COMBINATORIA DE UN VECTOR DE STRINGS
fn combine(elements: &Vec<String>, start: usize, result: &mut Vec<Vec<String>>) {
    let n = elements.len();
    for i in start..n {
        for j in start..n {
            if i != j {
                result.push(vec![elements[i].clone(), elements[j].clone()]);
            }
        }
    }
}

//FUNCION QUE OBTIENE LOS VALORES UNICOS DE CADA HEADER
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

//FUNCION QUE LEE UN CSV Y LO GUARDA EN UN HASHMAP
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