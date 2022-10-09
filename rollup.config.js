import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";
import { terser } from "rollup-plugin-terser";

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

        is_watch && serve({
            contentBase: "shylock-dominator/dist",
            open: true,
        }),

        is_watch && livereload("shylock-dominator/dist"),

        !is_watch && terser(),
    ],
};
