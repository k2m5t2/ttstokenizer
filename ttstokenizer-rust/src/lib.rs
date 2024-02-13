#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use console_error_panic_hook;
use std::panic;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use grapheme_to_phoneme::Model; // prediction-based g2p model
use grapheme_to_phoneme::GraphToPhoneError;
use cmudict_fast::Cmudict; // CMU pronunciation dictionary
// use cmudict::Cmudict;
use std::fs;
use std::collections::BTreeMap;

// TODO modularize everything into different files
// (unless configuring import structure is a sizeable headache, like in Python)

// read ljspeech_config.yaml
use serde::{Serialize, Deserialize};
use serde_yaml;

#[wasm_bindgen]
pub fn init_panic_hook() {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
}

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

struct PhonemeTokenIndex {
  token_idx: i32,
  phoneme: String,
}

struct G2PConverter {
  cmudict: Cmudict,
  g2p_model: Model,
}

impl G2PConverter {
  // TODO make the cmudict path argument optional (i.e., override?)
  fn new(mut cmudict_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
    // initiate cmudict & g2p model
    include_str!("./cmudict.dict");
    // if cmudict_path.is_empty() { cmudict_path = "./src/cmudict.dict"; }
    if cmudict_path.is_empty() { cmudict_path = "./src/cmudict.dict"; }
    let cmudict = Cmudict::new(cmudict_path)?;
    // TODO make it download from the internet
    let g2p_model = Model::load_in_memory()?;
    Ok(Self { cmudict, g2p_model })
  }

  fn convert(&self, text: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Attempt to convert using cmudict first
    if let Some(result) = self.cmudict.get(text) { // NOTE sample output of cmudict.get() -> Some([Rule { label: "abounds", pronunciation: [AH(None), B, AW(Primary), N, D, Z] }])
      let pronunciation = result.first().unwrap().pronunciation(); // NOTE first() must be omitted if using original cmudict library instead of cmudict_fast
      return Ok(pronunciation.iter().map(|p| p.to_string()).collect());
    }
    else {
      // Fallback to using the prediction model
      let result = self.g2p_model.predict_phonemes_strs(text)?;
      return Ok(result.iter().map(|&p| p.to_string()).collect())
      // TODO - check whether G2P model can just use .join() ?
    }
    // TODO currently this returns 
  }

  // fn pron_to_string(cmudict_fast_pron: cmudict_fast::Rule) -> String {
    fn pron_to_string(&self, cmudict_fast_pron: &[cmudict_fast::Symbol]) -> String {
      // Convert Symbol to String or &str Before Joining: If Symbol does not directly support join, you may need to map each Symbol to a String or &str first. 
      let pron_str = cmudict_fast_pron
        .iter()
        .map(|symbol| symbol.to_string()) // Convert each Symbol to String
        .collect::<Vec<String>>() // Collect into Vec<String>
        .join(" "); // Now you can use join
      return pron_str;
  }
}

struct Normalizer {

}

struct Tokenizer {
  tts_config: Config,
  phoneme_map: Vec<PhonemeTokenIndex>
}

impl Tokenizer {
  fn new(mut tts_config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
    if tts_config_path.is_empty() { tts_config_path = "./src/ljspeech_config.yaml"; }
    // read tts (e.g., ljspeech) config file (which maps phoneme into token indices)
    let tts_config_file: String = fs::read_to_string(tts_config_path).expect("Couldn't read TTS config file");
    let tts_config: Config = serde_yaml::from_str(&tts_config_file).expect("Couldn't interpret TTS config file"); 
    // print!{"{:?}", ljspeech_config}; // DEBUG phoneme list: ljspeech_config.token.list

    let mut phoneme_map: Vec<PhonemeTokenIndex> = Vec::new();

    for (idx, val) in tts_config.token.list.iter().enumerate() {
      phoneme_map.push(PhonemeTokenIndex {
        token_idx: idx as i32,
        phoneme: val.to_string(),
      });
    }
    Ok(Self { tts_config, phoneme_map })
  }

  // fn to_token_indices(&self, phonemes: Vec<&str>) -> Vec<i32> {
  fn to_token_indices(&self, phonemes: Vec<String>) -> Vec<i32> {
    let mut token_output: Vec<i32> = Vec::new();
    
    for phoneme in phonemes {
      for token in &self.phoneme_map {
        if token.phoneme == phoneme.to_string() {
          token_output.push(token.token_idx);
        }
      }
    }
    // print!{"{:?}", token_output}; // DEBUG
    return token_output;
  }
}

#[wasm_bindgen]
pub struct Processor {
  g2p_converter: G2PConverter,
  tokenizer: Tokenizer,
}

#[wasm_bindgen]
impl Processor {
  pub fn new() -> Processor {
    let g2p_converter = G2PConverter::new("").expect("Failed to create G2PConverter");
    let tokenizer = Tokenizer::new("").expect("Failed to create Tokenizer");
    Self { g2p_converter, tokenizer }
  }

  #[wasm_bindgen] // NOTE not sure if needed - top-level struct only or methdos too?
  // pub fn convert(&self, text: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
  pub fn process(&self, text: &str) -> JsValue {
    let phonemes = self.g2p_converter.convert(text);
    match phonemes {
      Ok(phonemes) => {
        let token_indices = self.tokenizer.to_token_indices(phonemes);
        // return JsValue::from_serde(&token_indices).unwrap();
        return serde_wasm_bindgen::to_value(&token_indices).unwrap_or(JsValue::UNDEFINED);
      },
      Err(e) => {
        return JsValue::from_str(&format!("Error converting text: {:?}", e));
      }
    }
    // let phonemes = self.g2p_converter.convert(text);
    // let token_indices = self.tokenizer.to_token_indices(phonemes);
  }
}

fn test() {
  // initiate G2P converter & tokenizer
  let g2p_converter = G2PConverter::new("").unwrap();
  let tokenizer = Tokenizer::new("").unwrap();
  
  // test a known word (e.g., abounds) using cmudict
  let known_pron = g2p_converter.convert("abounds").unwrap();
  print!{"{:?}", known_pron}; 
  let known_indices = tokenizer.to_token_indices(known_pron);
  print!{"{:?}", known_indices}; 
  
  // test an unknown word (e.g., kimchi-jjigae) using prediction model
  let unknown_pron = g2p_converter.convert("kimchi-jjigae").unwrap();
  print!{"{:?}", unknown_pron}; 
  let unknown_indices = tokenizer.to_token_indices(unknown_pron);
  print!{"{:?}", unknown_indices}; 
}

fn comparison_report() {
  let g2p_converter = G2PConverter::new("").unwrap();

  // test equality / relationship of different G2P methods
  let test_file: String = fs::read_to_string("./test/words.yaml").expect("Couldn't read word test file");
  let word_collection: WordList = serde_yaml::from_str(&test_file).expect("Couldn't deserialize word test file"); 

  let wordlength: usize = word_collection.words.len();
  // let comparisons: vec![Comparison; wordlength]; // NOTE is it possible/better to make a fixed-sized vector/array?
  let mut comparisons: Vec<Comparison> = Vec::new();

  for word in word_collection.words {
    let result_1 = g2p_converter.cmudict.get(&word);    
    let pronunciation_1 = result_1
      .and_then(|r| r.first()) // Use and_then to access the first item if result_1 is Some
      .map(|pronunciation| g2p_converter.pron_to_string(pronunciation.pronunciation())) // Transform pronunciation to a String
      .unwrap_or_else(|| "DNE".to_string()); // Provide an empty string if None

    let result_2 = g2p_converter.g2p_model.predict_phonemes_strs(&word);
    let pronunciation_2 = match &result_2 {
      Ok(pronunciation_2) => pronunciation_2.join(" "),
      Err(_) => "DNE".to_string(),
    };

    let comparison = Comparison {
      original: word,
      pronunciation_1: pronunciation_1,
      pronunciation_2: pronunciation_2,
    };

    comparisons.push(comparison);
}

  let test_result_yaml = serde_yaml::to_string(&comparisons).unwrap();
  fs::write("./test/test_results_2.txt", test_result_yaml).unwrap();

}

fn main() {
  test();
  comparison_report();
}