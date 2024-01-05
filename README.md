# rs-node

> TypeScript execution for node.js, focused on performance

### Overview

[`ts-node`](https://github.com/TypeStrong/ts-node) always feels slow and clunky to me. I want to use TypeScript and I want executing it to be fast and easy. Since most IDE's handle the type checking for me, my desire was always to have a blazing fast executor that wasn't actually concerned with type validation.

In addition to that, I've been wanting to play around with rust. Ba da bing, a perfect project.

Enter, `rs-node` - A rust implemented, blazing fast, typescript polyfiller.

### Usage

```bash
# Execute a script as `node` + `tsc`.
ts-node script.ts
```
