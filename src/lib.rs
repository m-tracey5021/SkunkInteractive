
use skunk::language::{construction::{parsing::Parser, program::{invoke_function, Program}}, components::{symbol::Symbol as SkSymbol, indexed_expression::IndexedExpression}, interpretation::evaluation::Result};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct DataSet {

    pub data: Vec<Vec<i32>>
}

#[wasm_bindgen]
pub fn sk_run(input: &str) -> String {
    
    let output = Program::new().run_with_output(&input.to_string());

    let mut output_str = String::new();

    for output_ln in output {

        output_str.push_str(&output_ln);

        output_str.push_str(&"\n");
    }  
    return output_str;  
}

#[wasm_bindgen]
pub fn sk_invoke(function_name: &str, parameters: &JsValue) -> String {
    
    let parameters: Vec<String> = parameters.into_serde().unwrap();

    let mut parser = Parser::new();

    let mut parsed_parameters = Vec::new();

    for parameter in parameters.iter() {

        let parsed = parser.parse_expression(&parameter);

        parsed_parameters.push(parsed);  
    }
    let parameters_indexed: Vec<IndexedExpression> = parsed_parameters.iter().map(|param| param.at_root()).collect();

    return match invoke_function(&String::from(function_name), parameters_indexed) {

        Some(result) => {

            match result {

                Result::Success(result) => result.to_string(),

                Result::Failure(error) => error
            }
        },
        None => String::from("Function is not defined")
    };
}

#[wasm_bindgen]
pub fn sk_dataset(input: &str) -> JsValue { // convert skunk tuple to array of integers
    
    let mut parser = Parser::new();

    let parsed = parser.parse_expression(&input.to_owned());

    let mut dataset: Vec<Vec<i32>> = Vec::new();

    match parsed.at_root().node() {

        SkSymbol::Tuple => {

            for child in parsed.at_root().children() {

                match parsed.at(&child).node() {

                    SkSymbol::Tuple => {

                        let mut point = Vec::new();

                        for grandchild in parsed.at(&child).children() {

                            match parsed.at(&grandchild).numeric_value() {

                                Some(value) => point.push(value),
            
                                None => point.push(0)
                            }
                        }
                        dataset.push(point);
                    },
                    _ => panic!("Not a dataset")
                }
            }
            return JsValue::from_serde(&dataset).unwrap();
        },
        _ => panic!("Not a dataset")
    }
}