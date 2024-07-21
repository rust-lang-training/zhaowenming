const crypto = require("node:crypto");
const fsPromises = require("fs/promises");
async function digestFile(filename) {
const hasher = crypto.createHash("SHA256");
const fin = await fsPromises.open(filename, "r");
const buf = new Buffer.alloc(1024 * 64);
for (;;) {
    const { bytesRead } = await fin.read(buf, 0, buf.length, 
    null);
    if (bytesRead === 0) {
    break;
     }
    hasher.update(buf.subarray(0, bytesRead));
     }
    await fin.close();
    return hasher.digest("hex");
    }
    (async function() {
        const filenames = [
        "./path/to/file1.zip",
        "./path/to/file2.zip",
        "./path/to/file3.zip",
        "./path/to/file4.zip",
         ];
        console.log("calculate file hash using JavaScript");
        for (let f of filenames) {
        const stat = await fsPromises.stat(f);
        const startMs = Date.now();
        const ret = await digestFile(f);
        const endMs = Date.now();
        const speed = (stat.size / 1024 / 1024) * 1000 / (endMs
        - startMs);
        console.log(`file size: ${stat.size} bytes, time used: 
        ${Date.now() - startMs} ms, speed: ${speed.toFixed(2)} MB/s, 
        ret: ${ret}`);
         }
         console.log("calculate file hash using Rust");
        })();