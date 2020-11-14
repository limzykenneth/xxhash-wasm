const path = require("path");
const DefinePlugin = require("webpack").DefinePlugin;
const WebpackDynamicPublicPathPlugin = require("webpack-dynamic-public-path");
const version = require("./package.json").version;

let publicPath = null;
if(process.env.CDN_BUILD === "true"){
	process.env.ASSETS_PATH = `https://cdn.jsdelivr.net/npm/twoxhash-wasm@${version}/dist/`;
}

module.exports = {
	entry: "./index.js",
	output: {
		path: path.resolve(__dirname, "dist"),
		filename: "twoxhash-wasm.js",
		library: "twoxhash",
		libraryTarget: "umd",
		chunkFilename: "bundle.js",
		webassemblyModuleFilename: "twoxhash.wasm"
	},
	plugins: [
		new DefinePlugin({
			"process.env.ASSETS_PATH": JSON.stringify(process.env.ASSETS_PATH)
		})
	],
	mode: "development"
};
