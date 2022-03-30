const fs = require("fs");
global.Wasabi = require('./node_modules/opencv-wasm/opencv.wasabi.js');

let analysis = require('./../../analysis.js');

const { cv, cvTranslateError } = require('opencv-wasm');

let mat = cv.matFromArray(2, 3, cv.CV_8UC1, [1, 2, 3, 4, 5, 6]);
console.log('cols =', mat.cols, '; rows =', mat.rows);
console.log(mat.data8S);

cv.transpose(mat, mat);
console.log('cols =', mat.cols, '; rows =', mat.rows);
console.log(mat.data8S);

require("./../../collect-data.js"); 

