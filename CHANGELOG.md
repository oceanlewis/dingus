# Changelog
All notable changes to this project will be documented in this file.

## [0.3.6]
### Changed
- config files no longer need to be specified with a '.yaml' extension

## [0.3.7]
### Changed
- aliased `session` subcommand to `shell`

## [0.3.8]
### Changed
- Dingus now tries to infer subcommands. e.g. `dingus s -c example` might be inferred as `dingus session -c example`
- Handle errors more or less properly