import {nodeResolve} from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import copy from "rollup-plugin-copy";
import json from "@rollup/plugin-json";
import {minify, swc} from 'rollup-plugin-swc3';
// @ts-ignore
import packageJson from "./package.json"

/**
 * @type {import('rollup').RollupOptions}
 */
export default {
    input: `src/loader/main.js`,
    output: {
        dir: `dist/`
    },
    plugins: [
        commonjs(),
        nodeResolve(),
        json({
            compact: true,
            preferConst: true
        }),
        swc({
            include: `./src/loader/*`,
            exclude: /node_modules/,
            tsconfig: false
        }),
        copy({
            targets: [{
                src: `pkg/${packageJson.name.replaceAll("-", "_")}_bg.wasm`,
                dest: `dist`
            }]
        }),
        minify({
            compress: {
                dead_code: true,
                defaults: true,
                arguments: true,
                booleans_as_integers: true,
                expression: true,
                module: true,
                passes: 3
            },
            format: {
                comments: false
            },
            keepClassnames: false,
            ecma: 2016,
            safari10: false,
            sourceMap: true,
            inlineSourcesContent: true
        })
    ]
}