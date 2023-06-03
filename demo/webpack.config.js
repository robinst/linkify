const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
  entry: "./web/index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
    // Fix CI build issue with Node 18. Can be removed when upgrading to Webpack 5, see https://stackoverflow.com/a/73465262
    hashFunction: "xxhash64",
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "./web/index.html",
      inject: true,
      filename: "index.html",
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "."),
    }),
    new CopyPlugin({
      patterns: [
        { from: "web/img", to: "img" },
      ],
    }),
  ],
  mode: "development",
};
