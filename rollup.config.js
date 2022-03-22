
import nodePolyfills from 'rollup-plugin-node-polyfills';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import globals from 'rollup-plugin-node-globals';
import json from '@rollup/plugin-json';

export default {
	input: 'lib/browser-index.js',
	output: {
		file: 'dist/bundle.js',
		format: 'umd',
		name: 'nearApi',
		// globals: {'Buffer': 'buffer'},
	},
	// external: ['buffer'], // <-- suppresses the warning
	plugins: [
		commonjs(),
		globals(),
		nodeResolve({ preferBuiltins: false, mainFields: ['browser'], browser: true }),
		nodePolyfills(),
		json()
	]
};
