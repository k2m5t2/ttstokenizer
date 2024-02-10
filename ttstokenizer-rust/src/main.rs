#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use grapheme_to_phoneme::Model;
use grapheme_to_phoneme::GraphToPhoneError;
use cmudict_fast::Cmudict;
// use cmudict::Cmudict;
use std::fs;
use std::collections::BTreeMap;

// read ljspeech_config.yaml
use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
  token: Token,
}

#[derive(Serialize, Deserialize, Debug)]
struct Token {
  list: Vec<String>,
}



fn main() {
  // // G2P using cmudict

  // initiate
  let dict = Cmudict::new("./src/cmudict.dict").expect("Couldn't make Cmudict");

  // convert
  // let result = dict.get("text to speech").unwrap().pronunciation();
  // let result = dict.get("text to speech"); // NOTE not *directly* available in dict; fails is_some() assertion

  let result = dict.get("abounds");
  assert!(result.is_some());

  // fetch
  let pronunciation = result.unwrap().first().unwrap(); // cmudict_fast library
  // let pronunciation = result.unwrap().pronunciation(); // original cmudict library

  print!{"{:?}", pronunciation};

  // // G2P using prediction model (backup)
  let result = convert_to_phoneme("text to speech");
  // assert_eq!(result.expect("should encode"),
  //   vec!["T", "EH1", "S", "T"].iter()
  //     .map(|s| s.to_string())
  //       .collect::<Vec<String>>());    

  
  // // read ljspeech config file (which maps phoneme into token indices)
  // let file_path: &str = "./src/ljspeech_config.yaml";
  // let ljspeech_config = fs::read_to_string(file_path).expect("Couldn't read ljspeech_config");
  // // let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&file_path)?; 
  // // let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&file_path).unwrap(); // 
  // // let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&ljspeech_config).expect("Couldn't make ljspeech_config"); 
  // let deserialized_map: Config = serde_yaml::from_str(&ljspeech_config).expect("Couldn't make ljspeech_config"); 
  
  // print!{"{:?}", deserialized_map};

  // map phoneme to token indices then output
  
  
}

fn convert_to_phoneme(text: &str) {
  let model = Model::load_in_memory()
    .expect("should load");

  let result: Result<Vec<&'static str>, GraphToPhoneError> = model.predict_phonemes_strs(text);

  // return result;
  print!{"{:?}", result};
}


fn to_token_indices(phonemes: &str) -> Vec<i32> {
  // TODO
  let result = Vec::new();
  return result;
}