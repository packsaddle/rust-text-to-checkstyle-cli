const spawn = require('cross-spawn');
const targets = [
  'i686-pc-windows-msvc',
  'x86_64-pc-windows-msvc',
  'i686-unknown-linux-musl',
  'x86_64-unknown-linux-musl',
  'i686-apple-darwin',
  'x86_64-apple-darwin',
];
const target1 = 'x86_64-unknown-linux-musl';

spawn.sync('rustup', ['target', 'add', target1], { stdio: 'inherit' });
spawn.sync('cargo', ['build', '--release', '--target', target1], { stdio: 'inherit' });
