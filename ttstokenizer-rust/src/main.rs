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

#[derive(Serialize, Deserialize, Debug)]
struct WordList {
  words: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Comparison {
  original: String,
  pronunciation_1: String,
  pronunciation_2: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestResults {
  words: Vec<Comparison>,
}

// fn pron_to_string(cmudict_fast_pron: cmudict_fast::Rule) -> String {
fn pron_to_string(cmudict_fast_pron: &[cmudict_fast::Symbol]) -> String {
    // Convert Symbol to String or &str Before Joining: If Symbol does not directly support join, you may need to map each Symbol to a String or &str first. 
    let pron_str = cmudict_fast_pron
      .iter()
      .map(|symbol| symbol.to_string()) // Convert each Symbol to String
      .collect::<Vec<String>>() // Collect into Vec<String>
      .join(" "); // Now you can use join
    return pron_str;
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
  // print!{"{:?}", result}; // Some([Rule { label: "abounds", pronunciation: [AH(None), B, AW(Primary), N, D, Z] }])

  // fetch
  let pronunciation = result.unwrap().first().unwrap(); // cmudict_fast library
  // let pronunciation = result.unwrap().pronunciation(); // original cmudict library

  // print!{"{:?}", pronunciation}; // Rule { label: "abounds", pronunciation: [AH(None), B, AW(Primary), N, D, Z] }
  print!{"{:?}", pronunciation.pronunciation()}; 
  print!{"{:?}", pronunciation.pronunciation()[0]};

  let firstPhoneme = &pronunciation.pronunciation()[0]; // ERROR due to borrowing


  // // G2P using prediction model (backup)
  let result = convert_to_phoneme("abounds");
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
  
  print!{"{:?}", "123"};

  // test equality / relationship of different G2P methods
  let test_file: String = fs::read_to_string("./test/words.yaml").expect("Couldn't read word test file");
  let word_collection: WordList = serde_yaml::from_str(&test_file).expect("Couldn't deserialize word test file"); 

  let wordlength: usize = word_collection.words.len();
  // let comparisons: vec![Comparison; wordlength];
  let mut comparisons: Vec<Comparison> = Vec::new();

  for word in word_collection.words {
    let result_1 = dict.get(&word);
    // assert!(result_1.is_some());
    // let pronunciation_1 = result_1.unwrap().first().unwrap(); // cmudict_fast library // NOTE fails upon unwrap() if word is not contained in cmudict
    
    let pronunciation_1 = result_1
      .and_then(|r| r.first()) // Use and_then to access the first item if result_1 is Some
      .map(|pronunciation| pron_to_string(pronunciation.pronunciation())) // Transform pronunciation to a String
      .unwrap_or_else(|| "DNE".to_string()); // Provide an empty string if None

    print!{"{:?}", pronunciation.pronunciation()};
    let result_2 = convert_to_phoneme(&word);
    print!{"{:?}", result};

    let comparison = Comparison {
      original: word,
      pronunciation_1: pronunciation_1,
      pronunciation_2: result_2.unwrap().join(" "),
    };

    comparisons.push(comparison);
    
  }

  let test_result_yaml = serde_yaml::to_string(&comparisons).unwrap();
  fs::write("./test/test_results.txt", test_result_yaml).unwrap();

}

fn convert_to_phoneme(text: &str) -> Result<Vec<&'static str>, GraphToPhoneError> {
  let model = Model::load_in_memory()
    .expect("should load");

  let result: Result<Vec<&'static str>, GraphToPhoneError> = model.predict_phonemes_strs(text);

  return result;
  // print!{"{:?}", result};
}


fn to_token_indices(phonemes: &str) -> Vec<i32> {
  // TODO
  let result = Vec::new();
  return result;
}