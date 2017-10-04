const spawn = require('cross-spawn');
const pify = require('pify');

pify(spawn).spawn('echo', ['yahoooo'], { stdio: 'inherit' }).then(data => {
  console.log(data);
});
