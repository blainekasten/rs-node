const glob = require('glob');
const fs = require('fs/promises');
const ffi = require('ffi-napi');
const path = require('path');

const lib = ffi.Library(path.join(__dirname, './target/release/libffi'), {
  require: ['string', ['string']],
});

glob.sync('./test_fixtures/**/*.in.ts').forEach((file) => {
  it(file.replace(/(test_fixtures\/|in.ts)/, ''), async () => {
    const output = await fs.readFile(file.replace(/in\..*/, 'out.js'), 'utf8');
    expect(lib.require(path.join(__dirname, file))).toBe(output.trim());
  });
});
