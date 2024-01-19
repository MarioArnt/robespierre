![Logo](https://raw.githubusercontent.com/MarioArnt/robespierre/master/robespierre.svg?sanitize=true)

[![npm](https://img.shields.io/npm/v/robespierre)](https://www.npmjs.com/package/robespierre)
![npm](https://img.shields.io/npm/dm/robespierre)
[![semantic-release](https://img.shields.io/badge/semantic--release-enabled?logo=semantic-release)](https://github.com/semantic-release/semantic-release)


Robespierre is a tool built in RUST and packaged via NPM that helps you find extraneous and implicit dependencies by 
comparing your manifest and actual imports in your **typescript** code using AST parsing.

* An *extraneous* dependency is a dependency declared in you `package.json` manifest, but unused in you code
* An *implicit* dependency is the opposite. A dependecy used somewhere in your codebase via an import statement, but not declared in your `package.json`. It can work somehow, 'cause it's installed indirectly by another dependency, but this is a bad practice.

Robespierre is focused on performance and uses [SWC](https://github.com/swc-project/swc) typescript AST parser to browse your codebase efficiently.

> The name "robespierre" is a humorous reference to Maximilien de Robespierre, a significant political figure in the French Revolution. Known for prolifically using the guillotine, our binary borrows its name because it helps you figure out which dependency's head to cut off!

## :arrow_down: Installation

```bash
# Via npm
npm install robespierre --save-dev

# Via yarn
yarn add -D robespierre

# Via pnpm
pnpm add -D robespierre
```

## :page_facing_up: Usage

```bash
# List available commands
robespierre --help

# Run robespierre on current dir
robespierre

# Run robespierre and write a json summary
robespierre --report
```

## :crab: Build

You can build the Robespierre Rust executable yourself, therefore you will need a working Rust and Cargo setup.

In order to compile for your native platform, just run:

```bash
cargo build --release
```

## :package: Packaging

You can then wrap the binary manually

```bash
export BUILD_OS=drawin
export BUILD_ARCH=arm64
export BUILD_NAME="${BUILD_OS}-${BUILD_ARCH}"
export BUILD_VERSION=1.0.12
export BUILD_TARGET=/path/to/compiled/robespierre

./package.sh
```

## :rocket: Release

To publish a new version on NPM use the script to version and tag to trigger a Github workflow.

```bash
export RELEASE_VERSION=1.0.12
sh ./version.sh ${RELEASE_VERSION}
git commit -am"chore: publish ${RELEASE_VERSION} :tada:"
```

> When merged on main branch, the version will be published on NPM and git tag will be created  
