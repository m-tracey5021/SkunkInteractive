
pub mod tests;

use skunk::language::{construction::{parsing::Parser, program::{invoke_function, Program}}, components::{symbol::Symbol as SkSymbol, indexed_expression::IndexedExpression}, interpretation::evaluation::SkResult};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct DataSet {

    pub dimensions: usize,

    pub data: Vec<Vec<i32>>
}

#[wasm_bindgen]
pub fn sk_run(input: &str) -> JsValue {
    
    let output = Program::new().run_with_output(&input.to_string());

    return JsValue::from_serde(&output).unwrap();

    // let mut output_str = String::new();

    // for output_ln in output {

    //     output_str.push_str(&output_ln);

    //     output_str.push_str(&"\n");
    // }  
    // return output_str;  
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

                SkResult::Success(result) => result.to_string(),

                SkResult::Failure(error) => error
            }
        },
        None => String::from("Function is not defined")
    };
}

#[wasm_bindgen]
pub fn sk_dataset(input: &str) -> JsValue { // convert skunk tuple to array of integers

    let mut parser = Parser::new();

    let parsed = parser.parse_expression(&input.to_owned());

    let tuple_transformed = parsed.at_root().to_axes();

    let dimensions = tuple_transformed.at_root().children().len();

    let mut data: Vec<Vec<i32>> = Vec::new(); 

    for axis in tuple_transformed.at_root().children() {

        let mut axis_values = Vec::new();

        for grandchild in tuple_transformed.at(&axis).children() {

            match tuple_transformed.at(&grandchild).numeric_value() {

                Some(value) => axis_values.push(value),

                None => axis_values.push(0)
            }
        }
        data.push(axis_values);
    }
    let dataset = DataSet { data: data, dimensions: dimensions };

    return JsValue::from_serde(&dataset).unwrap();
}
