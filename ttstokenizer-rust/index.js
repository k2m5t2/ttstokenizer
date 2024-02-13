// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "./pkg/ttstokenizer_rust.js";
// import { Processsor } from "./pkg/ttstokenizer_rust.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const ttstokenizer = await init("./pkg/ttstokenizer_rust_bg.wasm");
  console.log(ttstokenizer);

  ttstokenizer.init_panic_hook()
  
  //   const processor = await Processor.new();
  // const processor = await ttstokenizer.Processor.new();
  const processor = await ttstokenizer.processor_new();
  console.log(processor);
  // const result = processor.convert("Hello World!");
//   const result = ttstokenizer.processor_process("Hello World!");
//   console.log(result);

// //   Call the Add function export from wasm, save the result
// //   const tokens = ttstokenizer.

//   // Set the result onto the body
//   document.body.textContent = `Hello World! Tokens: ${result}`;
};
runWasm();


// import * as wasm from "./pkg/ttstokenizer_rust.js";
// // import { Processsor } from "./pkg/ttstokenizer_rust.js";

// async function init() {
//   // Instantiate our wasm module
//   //   const ttstokenizer = await init("./pkg/ttstokenizer_rust_bg.wasm");
//   await wasm.wasm_bindgen('./pkg/ttstokenizer_rust_bg.wasm');

//   // //   const processor = await Processor.new();
//   const processor = wasm.Processor.new();
//   const result = processor.convert("Hello World!");

// //   const tokens = ttstokenizer.

//   // Set the result onto the body
//   document.body.textContent = `Hello World! Tokens: ${result}`;
// };
// runWasm();