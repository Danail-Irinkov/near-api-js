import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import json from '@rollup/plugin-json';

export default {
	input: 'lib/index.js',
	output: {
		file: 'dist/bundle.js',
		format: 'umd',
		name: 'nearApi',
		// globals: {'@webassemblyjs/wasm-parser': 'wasmParser'},
	},
	// external: ['@webassemblyjs/wasm-parser'], // <-- suppresses the warning
	plugins: [commonjs(), nodeResolve(), json()]
};
