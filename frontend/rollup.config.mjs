import typescript from '@rollup/plugin-typescript';
import terser from '@rollup/plugin-terser';
import scss from 'rollup-plugin-scss';

// rollup.config.js
export default {
	input: 'js/index.ts',
	output: [{
		file: 'build/bundle.js',
		format: 'cjs'
    },{
        file: '../static/bundle.min.js',
		format: 'iife',
		name: 'version',
		plugins: [terser()]
    }],
    plugins: [typescript(),
             scss({ 
                fileName: "bundle.min.css",
                output: "../static/bundle.min.css",
                outputStyle: "compressed",
             })]
};
