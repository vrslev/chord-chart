const sh = require("shelljs");

sh.rm("-rf", "pkg.*");

for (const [target, dir] of [
  ["bundler", "pkg.bundler"],
  ["nodejs", "pkg.node"],
  ["web", "pkg.web"],
]) {
  sh.exec(`wasm-pack build --target ${target} --out-dir ${dir}`);
  sh.rm(`${dir}/{LICENSE,package.json,README.md,.gitignore}`);
}
