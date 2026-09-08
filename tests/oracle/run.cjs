'use strict';
const fs = require('fs');
const path = require('path');
process.env.NODE_PATH = path.resolve(__dirname,'node_modules');
require('module').Module._initPaths();
const ts = require('typescript');
const readline = require('readline');
const root = path.resolve(__dirname, '../../vendor/okta-auth-js');
const manifest = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../../translation.json')));
const entries = new Map(manifest.entries.map(e => [e.export_as || e.symbol, e]));
require.extensions['.ts'] = (module, filename) => {
  const source = fs.readFileSync(filename, 'utf8');
  const result = ts.transpileModule(source, { compilerOptions: {
    target: ts.ScriptTarget.ES2020, module: ts.ModuleKind.CommonJS,
    esModuleInterop: true,
  }, fileName: filename });
  // Expose selected private helpers to the test harness without changing their bodies.
  const expose = manifest.entries.filter(e => path.join(root,e.module) === filename && e.symbol !== 'default')
    .map(e => `\nmodule.exports[${JSON.stringify(e.symbol)}] = ${e.symbol};`).join('');
  module._compile(result.outputText + expose, filename);
};
function normalize(_, value) {
  if (ArrayBuffer.isView(value)) return Array.from(value);
  if (value instanceof ArrayBuffer) return Array.from(new Uint8Array(value));
  return value;
}
(async () => {
  for await (const line of readline.createInterface({input:process.stdin})) {
    try {
      const request = JSON.parse(line);
      const entry = entries.get(request.function);
      const fn = require(path.join(root, entry.module))[entry.symbol];
      const result = await fn(...request.args);
      console.log(JSON.stringify({ok: result === undefined ? null : result}, normalize));
    } catch (error) {
      console.log(JSON.stringify({error: {name:error.name || 'Error',message:error.message || String(error)}}));
    }
  }
})().catch(error => {console.error(error); process.exitCode = 1;});
