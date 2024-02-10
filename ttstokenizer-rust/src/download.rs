use reqwest;
use tempfile::TempDir;
use std::fs::OpenOptions;

// // download
// let dict = Cmudict::download()?; // NOTE only for original cmudict library
// download_cmudict();
// turn into token indices

// fn download_cmudict() -> Result<Cmudict, Error> {
// fn download_cmudict() -> Result<Cmudict> {
// fn download_cmudict() {
//   let tmpdir = TempDir::new("cmudict")?;
//   let path = tmpdir.path().join("cmudict.dict");
//   let mut file = OpenOptions::new().create(true).write(true).open(&path)?;
//   // let mut r = reqwest::get("https://raw.githubusercontent.com/cmusphinx/cmudict/master/cmudict.dict")?;
//   let mut r = reqwest::get("https://raw.githubusercontent.com/cmusphinx/cmudict/master/cmudict.dict");
//   r.copy_to(&mut file)?;
//   // return Cmudict::new(&path);
// }

// // NOTE ChatGPT's implementation
// use reqwest::blocking::get;
// use reqwest::Error;
// use std::io::copy;

// fn download_cmudict() -> Result<(), Box<dyn std::error::Error>> {
//     // let tmpdir = TempDir::new("cmudict")?;
//     let tmpdir = TempDir::new()?;
//     let path = tmpdir.path().join("cmudict.dict");
//     let mut file = OpenOptions::new().create(true).write(true).open(&path)?;

//     // Use the blocking client to make a synchronous request
//     let mut response = get("https://raw.githubusercontent.com/cmusphinx/cmudict/master/cmudict.dict")?;

//     // Check if the request was successful
//     if response.status().is_success() {
//         // Use copy_to to write the response body directly to the file
//         response.copy_to(&mut file)?;
//     } else {
//         return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Request failed")));
//     }

//     // Assuming you have a Cmudict::new function to work with, uncomment and adjust as necessary
//     return Ok(Cmudict::new(&path));
//     // Ok(())
// }