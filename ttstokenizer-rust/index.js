// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "./pkg/ttstokenizer_rust.js";
import { Processsor } from "./pkg/ttstokenizer_rust.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const ttstokenizer = await init("./pkg/ttstokenizer_rust_bg.wasm");

  const processor = await Processor.new();
  const result = processor.process("Hello World!");

//   Call the Add function export from wasm, save the result
//   const tokens = ttstokenizer.

  // Set the result onto the body
  document.body.textContent = `Hello World! Tokens: ${result}`;
};
runWasm();