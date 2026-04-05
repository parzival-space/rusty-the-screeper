"use strict";
import 'fastestsmallesttextencoderdecoder-encodeinto/EncoderDecoderTogether.min.js';
import * as wasm from '../../pkg';

// filename of the wasm binary
const WASM_NAME = "rusty_the_screeper_bg";

let running = false, wasm_bytes, wasm_module, wasm_instance;

function console_error() {
    const error_args = _
        .map(arguments, (arg) => (arg instanceof Error) ? arg.stack : arg);
    console.log("Error:", ...error_args);
    Game.notify(error_args.join(' '));
}

function loop_from_memory() {
    console.error = console_error;
    if (running) {
        // I don't know, just stole it.
        // workaround for https://github.com/rustwasm/wasm-bindgen/issues/3130
        Game.cpu.halt();
    } else {
        try {
            running = true;
            wasm_instance.tick();
            running = false;
        } catch (error) {
            console.error(`Caught exception, will halt next tick:`, error);
        }
    }
}

module.exports.loop = () => {
    console.error = console_error;

    if (!wasm_bytes) wasm_bytes = require(WASM_NAME);
    if (!wasm_module) wasm_module = new WebAssembly.Module(wasm_bytes);
    if (!wasm_instance) wasm_instance = wasm.initSync({module: wasm_module});

    // clean unnecessary memory
    wasm_bytes = null;
    delete require.cache[WASM_NAME];

    module.exports.loop = loop_from_memory;

    wasm_instance.init();
    console.log(`Module load completed, CPU used: ${Game.cpu.getUsed()}`);
}