#!/usr/bin/env node

const ffi = require('ffi-napi');
const path = require('path');

var lib = ffi.Library(path.join(__dirname, './target/release/libffi'), {
  require: ['string', ['string']],
});
require.extensions['.ts'] = require.extensions['.js'];

const originalRequire = require;
require('module').prototype.require = function (module) {
  const path = require.resolve(module);

  if (path.endsWith('.js')) {
    return originalRequire(path);
  }

  const str = lib.require(path);

  return eval(str);
};

const filePath = path.isAbsolute(process.argv[2])
  ? process.argv[2]
  : path.join(__dirname, process.argv[2]);

require(filePath);
