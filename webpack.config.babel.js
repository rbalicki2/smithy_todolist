const path = require('path');

console.log(process.env.NODE_ENV);

module.exports = {
  entry: "./dist/index.js",
  output: {
    path: path.resolve(__dirname, "dist_prod"),
    filename: "index.js",
  },
  mode: process.env.NODE_ENV,
};
