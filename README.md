<p align="center">
  <img src="./robespierre.svg?sanitize=true" alt="Logo" width=400 />
</p>

Robespierre is a tool built in RUST and packaged via NPM that helps you find extraneous and implicit dependencies by 
comparing your manifest and actual imports in your **typescript** code using AST parsing.

* An *extraneous* dependency is a dependency declared in you `package.json` manifest, but unused in you code
* An *implicit* dependency is the opposite. A dependecy used somewhere in your codebase via an import statement, but not declared in your `package.json`. It can work somehow, 'cause it's installed indirectly by another dependency, but this is a bad practice.

Robespierre is focused on performance and uses [SWC](https://github.com/swc-project/swc) typescript AST parser to browse your codebase efficiently.

> The name "robespierre" is a humorous reference to Maximilien de Robespierre, a significant political figure in the French Revolution. Known for prolifically using the guillotine, our binary borrows its name because it helps you figure out which dependency's head to cut off!

## Installation :arrow_down:

```bash
# Via npm
npm install robespierre --save-dev

# Via yarn
yarn add -D robespierre

# Via pnpm
pnpm add -D robespierre
```

## Usage :page_facing_up:

```bash
# List available commands
robespierre --help

# Run robespierre on current dir
robespierre

# Run robespierre and write a json summary
robespierre --report
```

## Build :crab:

You can build the Robespierre Rust executable yourself, therefore you will need a working Rust and Cargo setup.

In order to compile for your native platform, just run:

```bash
cargo +nightly build --release
```

## Packaging :package:

You can then wrap the binary (by using bash):

```bash
export OS=drawin
export ARCH=arm64

./package.sh
```

# Release :rocket:

To publish a new version on NPM use the script to version and tag to trigger a Github workflow.

```bash
export RELEASE_VERSION=1.0.12
sh ./version.sh ${RELEASE_VERSION}
git commit -am"chore: publish ${RELEASE_VERSION} :tada:"
git tag v${RELEASE_VERSION}
git push --tags
```

When branch is merged on main, the packages are built and publish on NPM.
