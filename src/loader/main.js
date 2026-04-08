"use strict";
import 'fastestsmallesttextencoderdecoder-encodeinto/EncoderDecoderTogether.min.js';
import * as wasm from '../../pkg';
import {name} from '../../package.json'

const WASM_NAME = `${name.replaceAll("-", "_")}_bg`;
let wasm_bytes, wasm_module, wasm_instance, running = false;

function consoleError(...args) {
    const errorArgs = args.map(arg => (arg instanceof Error) ? arg.stack : arg);
    console.log("Error:", ...errorArgs);
    Game.notify(errorArgs.join(' '));
}

function loop_from_memory() {
    console.error = consoleError;
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
    console.error = consoleError;

    if (!wasm_bytes) wasm_bytes = require(WASM_NAME);
    if (!wasm_module) wasm_module = new WebAssembly.Module(wasm_bytes);
    if (!wasm_instance) wasm_instance = wasm.initSync({module: wasm_module});

    // clean unnecessary memory
    wasm_bytes = null;
    delete require.cache[WASM_NAME];

    module.exports.loop = loop_from_memory;

    wasm_instance.init();
    global.wasm_instance = wasm_instance;
    console.log(`Module load completed, CPU used: ${Game.cpu.getUsed()}`);
}