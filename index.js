import ffi from 'ffi-napi';
import path from 'path';

var lib = ffi.Library(path.join(__dirname, './target/release/libffi'), {
  require: ['string', ['string']],
});
require.extensions['.ts'] = require.extensions['.js'];

function requirePatch() {
  const originalRequire = require;
  require('module').prototype.require = function (module) {
    const path = require.resolve(module);

    if (path.endsWith('.js')) {
      return originalRequire(path);
    }

    const str = lib.require(path);

    return eval(str);
  };
}

requirePatch('eslint');
const source = require('./test');
// console.log({ source });
