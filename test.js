const fs = require('fs');
const path = require('path');

const getMostRecentFile = (files) => {
  return files.length ? files[0].mtime : undefined;
};

const getMostOldestFile = (files) => {
  return files.length ? files[files.length - 1].mtime : undefined;
};

const orderReccentFiles = (dir) => {
  return fs
    .readdirSync(dir)
    .filter((file) => fs.lstatSync(path.join(dir, file)).isFile())
    .map((file) => ({ file, mtime: fs.lstatSync(path.join(dir, file)).mtime }))
    .sort((a, b) => b.mtime.getTime() - a.mtime.getTime());
};
let fss = orderReccentFiles('/home/Rust-Workers/out');

console.log(
  (getMostRecentFile(fss) - getMostOldestFile(fss)) / 1000 + ' seconds',
);
