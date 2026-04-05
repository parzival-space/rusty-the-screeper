const { nodeResolve } = require('@rollup/plugin-node-resolve');
const commonjs = require('@rollup/plugin-commonjs');
const babel = require('@rollup/plugin-babel');
const copy = require('rollup-plugin-copy');
const tenser = require('@rollup/plugin-terser');

module.exports = function (grunt) {
    const screepsAuth = require('./.screeps.json')

    grunt.loadNpmTasks('grunt-screeps');
    grunt.loadNpmTasks('grunt-rollup');

    grunt.config.init({
        screeps: {
            options: screepsAuth,
            dist: {
                files: [
                    {
                        expand: true,
                        src: ['dist/*.{js,wasm}'],
                        flatten: true
                    }
                ]
            }
        },
        rollup: {
            options: {
                plugins: [
                    commonjs(),
                    nodeResolve(),
                    babel({
                        babelHelpers: 'bundled',
                        presets: ['@babel/preset-env'],
                        targets: {
                            "node": 24,
                        },
                    }),
                    copy({
                        targets: [{
                            src: `pkg/rusty_the_screeper_bg.wasm`,
                            dest: 'dist',
                            rename: `rusty_the_screeper_bg.wasm`,
                        }]
                    }),
                    tenser(),
                ],
            },
            main: {
                dest: 'dist/main.js',
                src: 'src/loader/main.js'
            }
        }
    });

    grunt.registerTask('default', ['rollup', 'screeps']);
}