use std::{collections::HashMap, process::exit};



//CONSTANTES
const PATH: &str = "src/dataset.csv";
const MIN_SUPPORT: f64 = 0.5;

fn main() {
    let mut df: HashMap<String, Vec<String>> = HashMap::new();

    //ARCHIVO DE SALIDA (EL NOMBRE DEL ARCHIVO USARÁ EL NOMBRE DEL DATASET + FECHA Y HORA DE EJECUCIÓN)
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    //EXTRAEMOS EL NOMBRE DEL DATASET
    let dataset_name = PATH.split("/").last().unwrap().split(".").next().unwrap();
    let output_file = format!("src/{}_{}.txt", dataset_name, timestamp);




    //LEEMOS EL CSV
    let mut headers: Vec<String> = Vec::new();

    read_csv(&mut df, PATH, &mut headers);

    //OBTENEMOS LOS DIFERENTES VALORES DE CADA VECTOR
    let unique_values = get_unique_values(&df);

    //-------------------REGLAS DE ASOCIACIÓN-------------------//
    //COMBINATORIA DE LOS HEADERS
    let mut result_headers: Vec<Vec<String>> = Vec::new();
    combine(&headers, 0, &mut result_headers);
    println!("Combinatoria de los Headers: \n{:?}\n", result_headers);
    write_to_file(&format!("Combinatoria de los Headers: \n{:?}\n", result_headers), &output_file);


    println!("Valores únicos: \n{:?}\n\n", unique_values);
    write_to_file(&format!("Valores únicos: \n{:?}\n\n", unique_values), &output_file);

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
        write_to_file(&format!("Combinatoria de los valores únicos de NX:{:?} y NY:{:?}:\n", headers[0], headers[1]), &output_file);
        
        for value in &values {
            
            //OCURRENCIAS DE VALUE EN LA COLUMNA DE HEADER[0] Y HEADER[1] EN EL DATAFRAME
            let ocurrences_header_one = filter_occurrences(&df, &headers[0], &value[0]);
            let ocurrences_header_two = filter_occurrences(&df, &headers[1], &value[1]);
            let count_header_one = ocurrences_header_one.get(&headers[0]).unwrap().len();
            let count_header_two = ocurrences_header_two.get(&headers[1]).unwrap().len();
            let total_data = df.get(&headers[0]).unwrap().len();
            
            // CANTIDAD DE OCURRENCIAS DE LA COMBINACIÓN DE VALUE EN EL DATAFRAME (NX^Y)
            // let ocurrences_combined = recursive_filter_combined_occurrences(
            //     vec![&df, &df],
            //     vec![(headers[0].clone(), value[0].clone()), (headers[1].clone(), value[1].clone())]
            // );
            // let nx_y = ocurrences_combined.get(&headers[0]).unwrap().len();
            let nx_y = find_matching_rows(&df, vec![(headers[0].clone(), value[0].clone()), (headers[1].clone(), value[1].clone())]).len();

            //SOPORTE DE LA COMBINACIÓN DE VALUE
            let support = nx_y as f64 / total_data as f64;

            //CONFIANZA DE LA COMBINACIÓN DE VALUE
            let confidence = nx_y as f64 / count_header_one as f64;

            //CORRELACIÓN DE LA COMBINACIÓN DE VALUE
            let lift = (nx_y as f64 * total_data as f64) / (count_header_one as f64 * count_header_two as f64);

            println!("NX={}: {count_header_one}, NY={}: {count_header_two}, NX^Y:{nx_y}, S:{support}, C: {confidence}, L: {lift}", value[0], value[1]);
            write_to_file(&format!("NX={}: {count_header_one}, NY={}: {count_header_two}, NX^Y:{nx_y}, S:{support}, C: {confidence}, L: {lift}\n", value[0], value[1], count_header_one=count_header_one, count_header_two=count_header_two, nx_y=nx_y, support=support, confidence=confidence, lift=lift), &output_file);

            //CONSERVAR LAS REGLAS DE ASOCIACIÓN QUE CUMPLAN CON EL SOPORTE MÍNIMO
            if support >= MIN_SUPPORT {
                let regla = format!("{}:{}-{}:{}", headers[0], value[0],headers[1], value[1]);
                // println!("Regla: {regla}");
                reglas_asociacion.push(regla);
            }
        }
    }

    println!("\n\n");
    write_to_file("\n\n", &output_file);

    //REGLAS DE SEGUNDO, TERCER, etc, GRADO
    // println!("{:?}", reglas_asociacion);
    println!("Reglas que cumplen con el soporte mínimo:\n{:?}\n", reglas_asociacion);
    write_to_file(&format!("Reglas que cumplen con el soporte mínimo:\n{:?}\n", reglas_asociacion), &output_file);
    //AQUÍ EMPEZAMOS EL CÓDIGO QUE ITERARÁ SOBRE EL NESIMO GRADO DE REGLAS DE ASOCIACIÓN
    let num_headers = headers.len();


    for i in 2..num_headers {
        //EXTRAEMOS LOS VALORES Y HEADERS DE LAS REGLAS DE ASOCIACIÓN ANTERIORES
        //ESTO DEVUELVE UNA COLECCIÓN DE REGLAS DE ASOCIACIÓN DESAGREGADAS EN HEADERS Y VALORES
        let unbundled_rules: Vec<Vec<Vec<String>>> = get_rules_info(&reglas_asociacion);
        //CREAMOS LA COMBINATORIA DE LOS HEADERS-VALORES DE LAS REGLAS DE ASOCIACIÓN ANTERIORES
        //(UNA COMBINACION DE VEC<VEC<STRING>>) POR CADA REGLA DE ASOCIACIÓN
        let combined_rules = combine_superior_rules(&unbundled_rules);
        

        //IMPRIMIMOS LAS NUVEAS REGLAS DE ASOCIACIÓN EN UN FORMATO CONVENIENTE
        println!("\nReglas de asociación de grado {}: ", i);
        write_to_file(&format!("\nReglas de asociación de grado {}: \n", i), &output_file);
        let nomenclature = Vec::from(["NX", "NY"]);
        let mut new_reglas_asociacion: Vec<String> = Vec::new();
        
        for rule in &combined_rules {
            //FORMAMOS LA STRING TITUAR DE LA REGLA DE ASOCIACIÓN. TIPO: "NX:VALOR-NY:VALOR-NZ:VALOR"
            let mut rule_string = String::new();
            let mut indice:usize=0;
            for part in rule {
                
                rule_string.push_str(&format!("{}=(", nomenclature[indice]));
                for header_values in part {
                    rule_string.push_str(&format!("{}:{} - ", header_values[0], header_values[1]));
                }
                indice += 1;
                rule_string.truncate(rule_string.len() - 3);
                rule_string.push_str(") & ");
                
            }
            rule_string.truncate(rule_string.len() - 3);
            //IMPRIMIMOS LA REGLA DE ASOCIACIÓN
            println!("{}", rule_string);
            write_to_file(&format!("{}\n", rule_string), &output_file);
            //SO:
            
            //AHORA SÍ, CALCULAMOS LAS APARICIONES NX Y NY DE LA REGLA DE ASOCIACIÓN
            //PARA CALCULAR EL NX, RECORREMOS EL rule[0].
            let mut nx_ocurrences:u64 = 0;
            //PARA CALCULAR EL NY, RECORREMOS EL rule[1]
            let mut ny_ocurrences:u64 = 0;
            //A PRIORI, ESTE CICLO SE EJECUTA SOLO 2 VECES SIN IMPORTAR QUÉ.
            let nomenclature = Vec::from(["NX", "NY"]);
            let mut indice:usize=0;
            let mut string_result = String::new();
            for part in rule {
                //PARA CADA PAR HEADER-VALOR EN LA REGLA DE ASOCIACIÓN
                let mut set_of_headers: Vec<String> = Vec::new();
                let mut set_of_values: Vec<String> = Vec::new();
                for header_value in part{
                    set_of_headers.push(header_value[0].clone());
                    set_of_values.push(header_value[1].clone());
                }
                // println!("HEADERS: {:?}", set_of_headers);
                //CREAMOS EL VECTOR DE CONDICIONES PARA LA FUNCIÓN recursive_filter_combined_occurrences
                let mut conditions_vec: Vec<(String, String)> = Vec::new();
                for i in 0..part.len(){
                    conditions_vec.push((set_of_headers[i].clone(), set_of_values[i].clone()));
                }

                let result = find_matching_rows(&df, conditions_vec).len();

                if indice == 0 {
                    nx_ocurrences = result as u64;
                } else {
                    ny_ocurrences = result as u64;
                }
                let format_string = format!("{}:{result}, ", nomenclature[indice]);                
                string_result.push_str(&format_string);
                indice += 1;
            }
            string_result.truncate(string_result.len() - 2);

            //CALCULAMOS EL NX^Y
            //OBTENEMOS LA LISTA DE PARES HEADER-VALOR DE LA REGLA DE ASOCIACIÓN
            let mut set_of_headers: Vec<String> = Vec::new();
            let mut set_of_values: Vec<String> = Vec::new();
            for part in rule{
                for header_value in part{
                    set_of_headers.push(header_value[0].clone());
                    set_of_values.push(header_value[1].clone());
                }
            }
            //CREAMOS EL VECTOR DE CONDICIONES PARA LA FUNCIÓN recursive_filter_combined_occurrences
            let mut conditions_vec: Vec<(String, String)> = Vec::new();
            for i in 0..set_of_headers.len(){
                conditions_vec.push((set_of_headers[i].clone(), set_of_values[i].clone()));
            }
            // println!("{:?}", conditions_vec);
            let ocurrences_combined = find_matching_rows(&df, conditions_vec);
            let nx_y = ocurrences_combined.len() as u64;
            //AGREGAMOS EL RESULTADO DE NX^Y A LA STRING DE RESULTADOS
            string_result.push_str(&format!(", NX^Y: {}", nx_y));

            //CALCULAMOS EL SOPORTE DE LA REGLA DE ASOCIACIÓN
            let total_data = df.get(&set_of_headers[0]).unwrap().len();
            let support = nx_y as f64 / total_data as f64;
            //AGREGAMOS EL RESULTADO DE SOPORTE A LA STRING DE RESULTADOS
            string_result.push_str(&format!(", S: {}", support));

            //CALCULAMOS LA CONFIANZA DE LA REGLA DE ASOCIACIÓN
            let confidence = nx_y as f64 / nx_ocurrences as f64;
            //AGREGAMOS EL RESULTADO DE CONFIANZA A LA STRING DE RESULTADOS
            string_result.push_str(&format!(", C: {}", confidence));

            //CALCULAMOS EL LIFT DE LA REGLA DE ASOCIACIÓN
            let lift = (nx_y as f64 * total_data as f64) / (nx_ocurrences as f64 * ny_ocurrences as f64);
            //AGREGAMOS EL RESULTADO DE LIFT A LA STRING DE RESULTADOS
            string_result.push_str(&format!(", L: {}", lift));

            //IMPRIMIMOS EL RESULTADO DE LAS APARICIONES DE NX Y NY
            println!("{}", string_result);
            write_to_file(&format!("{}\n", string_result), &output_file);

            //CONSERVAR LAS REGLAS DE ASOCIACIÓN QUE CUMPLAN CON EL SOPORTE MÍNIMO
            if support >= MIN_SUPPORT {
                let regla = format!("{}:{}-{}:{}", set_of_headers[0], set_of_values[0],set_of_headers[1], set_of_values[1]);
                // println!("Regla: {regla}");
                new_reglas_asociacion.push(regla);
            }

        }
        //ACTUALIZAMOS LAS REGLAS DE ASOCIACIÓN
        reglas_asociacion = new_reglas_asociacion;
        
        
        //CRITERIO DE PARO TEMPORAL: SI EL VECTOR DE PARES HEADER-VALOR DE LA REGLA DE ASOCIACIÓN ES IGUAL O MAYOR AL TOTAL DE HEADERS, SE DETIENE EL CICLO
        
        //CALCULAMOS EL SET_OF_HEADERS DE LA REGLA DE ASOCIACIÓN
        let mut set_of_headers: Vec<String> = Vec::new();
        for rule in &unbundled_rules {
            for header_value in rule{
                set_of_headers.push(header_value[0].clone());
            }
        }
        // println!("SET OF HEADERS: {:?}", set_of_headers);
        if set_of_headers.len() >= num_headers {
            println!("\nSe han encontrado todas las reglas de asociación posibles.");
            write_to_file("\nSe han encontrado todas las reglas de asociación posibles.", &output_file);
            exit(0);
        }
        // for rule in &unbundled_rules {
        //     //CADA RULE CONTIENE N HEADERS Y N VALORES.
        // }

    }


    
    
}
//FUNCIONES

//FUNCION QUE RECIVE UN VEC<VEC<VEC<STRING>>> Y DEVUELVE UN VEC<VEC<VEC<VEC<STRING>>>>
//TOMA ESE VEC<VEC<VEC<STRING>>> Y REALIZA LA COMBINATORIA DE CADA VEC<VEC<STRING>> Y DEVUELVE UN VEC<VEC<VEC<VEC<STRING>
fn combine_superior_rules(rules: &Vec<Vec<Vec<String>>>) -> Vec<Vec<Vec<Vec<String>>>> {
    let mut combined_rules: Vec<Vec<Vec<Vec<String>>>> = Vec::new();

    for i in 0..rules.len() {
        for j in i + 1..rules.len() {
            let mut combined_rule: Vec<Vec<Vec<String>>> = Vec::new();
            combined_rule.push(rules[i].clone());
            combined_rule.push(rules[j].clone());
            combined_rules.push(combined_rule);
        }
    }

    combined_rules
}

fn get_rules_info(rules: &Vec<String>) -> Vec<Vec<Vec<String>>> {
    let mut rules_values: Vec<Vec<Vec<String>>> = Vec::new();
    for rule in rules {
        let rule_header: Vec<String> = rule.split("-").map(|s| s.to_string()).collect();
        let mut rule_values: Vec<Vec<String>> = Vec::new();
        for header in &rule_header {
            let header_values: Vec<String> = header.split(":").map(|s| s.to_string()).collect();    
            rule_values.push(header_values);
        }
        rules_values.push(rule_values);
    }
    // println!("\nVALUES: {:?}", rules_values);
    rules_values
}

// FUNCION QUE DEVUELVE UN DATAFRAME CON LAS OCURRENCIAS DE UNA COMBINACIÓN DE VALORES EN DOS DATAFRAMES
fn _filter_combined_occurrences(
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

fn find_matching_rows(
    dataframe: &HashMap<String, Vec<String>>,
    conditions: Vec<(String, String)>,
) -> Vec<usize> {
    // Verificar que todas las columnas en condiciones existen en el dataframe
    for (header, _) in &conditions {
        if !dataframe.contains_key(header) {
            return vec![]; // Si falta alguna columna, no se puede encontrar ninguna coincidencia
        }
    }

    // Inicializar vector de índices coincidentes con los índices de la primera condición
    let (first_header, first_value) = &conditions[0];
    let initial_indices: Vec<usize> = dataframe[first_header]
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if v == first_value { Some(i) } else { None })
        .collect();

    // Filtrar índices iniciales con el resto de condiciones
    conditions.iter().skip(1).fold(initial_indices, |mut indices, (header, value)| {
        indices.retain(|&i| dataframe[header].get(i) == Some(value));
        indices
    })
}

fn _recursive_filter_combined_occurrences(
    dfs: Vec<&HashMap<String, Vec<String>>>,
    conditions: Vec<(String, String)>
) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();

    // Inicializa las columnas en el nuevo dataframe
    for (header, _) in &conditions {
        result.insert(header.clone(), Vec::new());
    }

    if !conditions.is_empty() {
        let (header, value) = &conditions[0];
        if let Some(values) = dfs[0].get(header) {
            for (i, val) in values.iter().enumerate() {
                if val == value {
                    let mut match_found = true;
                    for (j, (header_j, value_j)) in conditions.iter().enumerate().skip(1) {
                        if let Some(values_j) = dfs[j].get(header_j) {
                            if values_j.get(i) != Some(value_j) {
                                match_found = false;
                                break;
                            }
                        } else {
                            match_found = false;
                            break;
                        }
                    }
                    if match_found {
                        for (header_k, _) in &conditions {
                            result.get_mut(header_k).unwrap().push(dfs[0].get(header_k).unwrap()[i].clone());
                        }
                    }
                }
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


//FUNCIÓN QUE RECIBE UNA CADENA DE TEXTO Y LA ESCRIBE AL FINAL DE UN ARCHIVO, LO CREA SI NO EXISTE
fn write_to_file(text: &str, path: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    file.write_all(text.as_bytes()).unwrap();
}