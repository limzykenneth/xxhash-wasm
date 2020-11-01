const path = require("path");
const version = require("./package.json").version;

let publicPath = null;
if(process.env.CDN_BUILD === "true"){
	publicPath = `https://cdn.jsdelivr.net/npm/twoxhash-wasm@${version}/dist/`;
}

module.exports = {
	entry: "./index.js",
	output: {
		path: path.resolve(__dirname, "dist"),
		filename: "xxhash-wasm.js",
		library: "xxhash",
		libraryTarget: "umd",
		chunkFilename: "bundle.js",
		webassemblyModuleFilename: "xxhash.wasm",
		publicPath: publicPath || process.env.ASSETS_PATH || "/js/"
	},
	mode: "production"
};
