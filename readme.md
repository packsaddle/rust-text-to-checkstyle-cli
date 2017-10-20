# text2checkstyle_cli

[![crates version][crates-image]][crates-url] [![Travis-CI Status][travis-image]][travis-url] [![Appveyor Status][appveyor-image]][appveyor-url] ![license][license-image]

```
text2checkstyle_cli
cli for text to checkstyle.

USAGE:
    text2checkstyle [OPTIONS] [FILE]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --column <COLUMN>         [default: 0]
        --line <LINE>             [default: 0]
        --name <NAME>             [default: path/to/file]
        --severity <SEVERITY>     [default: info]
        --source <SOURCE>         [default: TextToCheckstyle]

ARGS:
    <FILE>
```

## changelog

[changelog](./changelog.md)

## License

MIT/Apache-2.0 © [sanemat](http://sane.jp)


[travis-url]: https://travis-ci.org/packsaddle/rust-text2checkstyle_cli
[travis-image]: https://img.shields.io/travis/packsaddle/rust-text2checkstyle_cli/master.svg?style=flat-square&label=travis
[appveyor-url]: https://ci.appveyor.com/project/sanemat/rust-text2checkstyle-cli/branch/master
[appveyor-image]: https://img.shields.io/appveyor/ci/sanemat/rust-text2checkstyle-cli/master.svg?style=flat-square&label=appveyor
[crates-url]: https://crates.io/crates/text2checkstyle_cli
[crates-image]: https://img.shields.io/crates/v/text2checkstyle_cli.svg?style=flat-square
[license-image]: https://img.shields.io/crates/l/text2checkstyle_cli.svg?style=flat-square
