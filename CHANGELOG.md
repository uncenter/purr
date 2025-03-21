# Changelog

## [1.2.1](https://github.com/uncenter/purr/compare/v1.2.0...v1.2.1) (2025-03-21)


### Bug Fixes

* **models:** remove categories field from ports data source, update categories list ([55653d1](https://github.com/uncenter/purr/commit/55653d170c13d584b36172be7d138dbcb99ce823))

## [1.2.0](https://github.com/uncenter/purr/compare/v1.1.0...v1.2.0) (2025-01-16)


### Features

* generic value cache support for whiskers statuses & stars ([#38](https://github.com/uncenter/purr/issues/38)) ([9b9753d](https://github.com/uncenter/purr/commit/9b9753dde5e6aa0e0801db21b97bdb3f1ca84e4f))
* **init/userstyle:** gate comment removal behind explicit `--clear-comments` flag ([913a339](https://github.com/uncenter/purr/commit/913a3391eb3f8930aa058d4c3db9ee2ee738e4ee))


### Bug Fixes

* **init/userstyle:** new template url/path, improve comment regex ([9a41fdc](https://github.com/uncenter/purr/commit/9a41fdcc87be6a7da060e9e60605127411ab90bd))

## [1.1.0](https://github.com/uncenter/purr/compare/v1.0.1...v1.1.0) (2024-11-12)


### Features

* **init/port:** support whiskers port template, remove .gitkeep file, update author name ([3440c8d](https://github.com/uncenter/purr/commit/3440c8d0c514e2b1d54f6225b519f3103cc38fda))

## [1.0.1](https://github.com/uncenter/purr/compare/v1.0.0...v1.0.1) (2024-11-12)


### Bug Fixes

* **query/stars:** fix `archived` flag behavior ([0262acf](https://github.com/uncenter/purr/commit/0262acf05b467b5d950c1930c2f5dfb2bb381b99))

## [1.0.0](https://github.com/uncenter/purr/compare/v0.5.0...v1.0.0) (2024-07-04)


### ⚠ BREAKING CHANGES

* merge `userstyles` subcommands into root `query` and `init` ([#34](https://github.com/uncenter/purr/issues/34))

### Features

* merge `userstyles` subcommands into root `query` and `init` ([#34](https://github.com/uncenter/purr/issues/34)) ([f39f861](https://github.com/uncenter/purr/commit/f39f8613445bdddc44c44abdfa71ab0b1533ef67))

## [0.5.0](https://github.com/uncenter/purr/compare/v0.4.1...v0.5.0) (2024-06-29)


### Features

* basic caching support ([#25](https://github.com/uncenter/purr/issues/25)) ([f109a65](https://github.com/uncenter/purr/commit/f109a65c3faf3a713bb1e723d0e650ff029f947c))

## [0.4.1](https://github.com/uncenter/purr/compare/v0.4.0...v0.4.1) (2024-06-18)


### Bug Fixes

* **query:** compare lowercase ids for --for ([4cc10f6](https://github.com/uncenter/purr/commit/4cc10f6c8b851a50d77e08a302240287becc2d86))

## [0.4.0](https://github.com/uncenter/purr/compare/v0.3.1...v0.4.0) (2024-06-10)


### Features

* **whiskerify:** handle hsl colors ([2fa76ea](https://github.com/uncenter/purr/commit/2fa76ea44c64dbb8c48e1e96f6b437dbd8e0fe90))


### Bug Fixes

* better handle invalid --for arguments ([81ad6d7](https://github.com/uncenter/purr/commit/81ad6d75de8e468cd1e053cf7e36a98e1c66859a))

## [0.3.1](https://github.com/uncenter/purr/compare/v0.3.0...v0.3.1) (2024-06-09)


### Bug Fixes

* **whiskerify:** don't only search for hex codes with # ([6104f93](https://github.com/uncenter/purr/commit/6104f933fba836ad026506362b8b44cd71c5a797))

## [0.3.0](https://github.com/uncenter/purr/compare/v0.2.1...v0.3.0) (2024-06-07)


### Features

* **whiskerify:** warn if original content to identical to output ([7e74694](https://github.com/uncenter/purr/commit/7e74694e78bb4d6a24e73608e268078f40b3df27))


### Bug Fixes

* **whiskerify:** use regex to match while ignoring hex casing ([a826ba5](https://github.com/uncenter/purr/commit/a826ba569fae27886a29e4ce1b2d2565e012739d))

## [0.2.1](https://github.com/uncenter/purr/compare/v0.2.0...v0.2.1) (2024-06-03)


### Bug Fixes

* use correct count flag variable ([7e2784c](https://github.com/uncenter/purr/commit/7e2784c79974ffe936b8d0f1180823bd87c36742))
