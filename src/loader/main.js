"use strict";
import * as wasm from '../../pkg';



const WASM_FILE = "rusty_the_screeper_bg";

let running = false, wasm_bytes, wasm_module, wasm_instance;

module.exports.loop = () => {
    if (!wasm_bytes) wasm_bytes = require(WASM_FILE);
    if (!wasm_module) wasm_module = new WebAssembly.Module(wasm_bytes);
    if (!wasm_instance) wasm_instance = wasm.initSync({ module: wasm_module });

    wasm_instance.init();
}