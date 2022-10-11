import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";
import { terser } from "rollup-plugin-terser";
import copy from 'rollup-plugin-copy';

const is_watch = !!process.env.ROLLUP_WATCH;

export default {
    input: {
        index: "./shylock-dominator/Cargo.toml",
    },
    output: {
        dir: "shylock-dominator/dist/js",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: "js/",
        }),

        copy({
            targets: [
                { src: 'node_modules/leaflet/dist/leaflet.js', dest: 'shylock-dominator/dist/js' },
                { src: 'node_modules/leaflet/dist/leaflet.css', dest: 'shylock-dominator/dist/css' },
                { src: 'node_modules/leaflet/dist/leaflet.js.map', dest: 'shylock-dominator/dist/js' },
                { src: 'node_modules/leaflet/dist/images/marker-shadow.png', dest: 'shylock-dominator/dist/css/images' },
                { src: 'node_modules/leaflet/dist/images/marker-icon.png', dest: 'shylock-dominator/dist/css/images' },
                { src: 'node_modules/leaflet/dist/images/marker-con-2x.png', dest: 'shylock-dominator/dist/css/images' }
            ]
        }),

        is_watch && serve({
            contentBase: "shylock-dominator/dist",
            open: true,
        }),

        is_watch && livereload("shylock-dominator/dist"),

        !is_watch && terser(),
    ],
};
