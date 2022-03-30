const fs = require("fs");
global.Wasabi = require('./node_modules/opencv-wasm/opencv.wasabi.js');

let analysis = require('./../../analysis.js');

const Jimp = require('jimp');
const { cv, cvTranslateError } = require('opencv-wasm');

(async () => {
    let jimpSrc = await Jimp.read(__dirname + '/input/image-sample-1.jpg');

    var src = cv.matFromImageData(jimpSrc.bitmap);

    let dst = new cv.Mat();
    let M = cv.Mat.ones(5, 5, cv.CV_8U);
    let anchor = new cv.Point(-1, -1);
    cv.dilate(src, dst, M, anchor, 1, cv.BORDER_CONSTANT, cv.morphologyDefaultBorderValue());

    new Jimp({
        width: dst.cols,
        height: dst.rows,
        data: Buffer.from(dst.data)
    })
        .write(__dirname + '/test-output/dilation.png');
})();

require("./../../collect-data.js")