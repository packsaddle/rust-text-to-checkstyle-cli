const spawn = require('cross-spawn');
const targets = [
  'i686-pc-windows-msvc',
  'x86_64-pc-windows-msvc',
];
for (let target of targets) {
  spawn.sync('rustup', ['target', 'add', target], { stdio: 'inherit' });
  spawn.sync('cargo', ['build', '--release', '--target', target], { stdio: 'inherit' });
}
