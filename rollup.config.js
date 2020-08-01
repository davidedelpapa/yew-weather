import livereload from "rollup-plugin-livereload";
import injectEnv from 'rollup-plugin-inject-env';

export default {
  input: "main.js",
  output: {
    file: "pkg/bundle.js",
    format: "iife",
    name: 'bundle',
  },
  plugins: [
    livereload('pkg'), 
    injectEnv()
  ],
};
