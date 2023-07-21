---
title: "Still using Node.js? Deno 1.0 is here!"
datetime: 2020-05-17T13:57:58.000Z
---

Ryan Dahl is a young software engineer. In 2009, he created Node.js - enabling
JavaScript anywhere, not only in browsers.

The main reason he did that was to build efficient event-driven HTTP servers.

It's undeniable that Node had a huge impact in software development, as it
quickly gained popularity. In fact, I bet you already know Node since tons of
companies adopted it for their backend development.

Even if Node was a big successful project Ryan regretted some of his decisions.
NPM, the primary package manager for Node and the central core of the Node
ecosystem, is a commercial company and it was acquired by Microsoft recently.
This doesn't necessarily means anything *bad* but it can affect the culture
behind the project.

After three years, Ryan quitted Node.js for good. It started using Go but at
the end it chose Rust for its new project: **Deno**.

In 2018 he gave a talk titled ["10 things I regret about
Node.js"](https://youtu.be/M3BM9TB-8yA). That's when Deno was announced, and
fast forward to some days ago: **Deno 1.0 arrived**.

**So what's that Deno thing?**

*"Deno is a **secure** runtime for **JavaScript** and **Typescript**"*
is the first sentence you read when you open
[https://deno.land](https://deno.land). What does that mean? Let's find out
together.

---

## 1. Secure

Ryan said *"your linter shouldn't be able to use your network"* and I think
that's interesting. 

When you run a script using Deno, you need to explicitly tell the permissions
it'll need.

I think of that as the permissions you give to phone apps nowadays.

```sh
# To run a script without permissions:
$ deno run main.js

# To run a script with some permissions:
$ deno run --allow-read=/usr main.js
```

It's pretty self explaining. I'm giving `main.js` the ability to read from
`/usr` folder. Not to write, not to read anything else.

If your script tries to do something you didn’t grant permissions for, you'd
get an error:

```sh
error: Uncaught PermissionDenied: read access to "/etc/passwd", run again with the --allow-read flag
```

The list of available permissions at this time is:
- **allow-read**, for reading from filesystem's paths
- **allow-write**, for writing to filesystem's paths
- **allow-env**, for reading environment variables
- **allow-net**, for accessing network resources (e.g. TCP sockets)

In my opinion, this permission whitelist is a great addition as you can (need)
to tune it for the script you want to run. That's definitely one of the core
points of Deno against Node.


## 2. Typescript

We know Node and Deno can run JS scripts, sometimes it's fine to have such a
dynamically-typed language as writing code is straightforward and fast. What
about when your codebase grows?

Unsurprisingly, Typescript grew a lot in the past few years as an alternative
to plain JavaScript for writing software at any scale. I don't have to sell you
type safety and compilation errors right? You can detect tons of problems
before even running your code.

Deno has first-class support for Typescript: it can run a `.ts` file natively.
Using `deno run somefile.ts` will trigger the compilation just before running
the code.

This means that writing Typescript has even less friction for developers. You
just write your file and run it (did someone said `go run`?).


## 3. Other goodies

Deno is packed with other interesting features as well! I won't cover
everything in this short blog post, you can find all of them in the official
manual: [https://deno.land/manual/](https://deno.land/manual/).

I think Deno was heavily inspired by Go for the external tools it provides to
developers, such as a formatter or the documentation generator. This will
certainly be the starting point for building editors, IDE, and other tools to
improve the developer experience when working with it.

I'll try to list some of the other features that I think are pretty cool:

### ES6 modules and module downloads

I didn't explicitly mention that before, but Deno doesn't use NPM or
`package.json`. The reason is to avoid centralizing package management.

Instead, Deno supports to run files from the internet. Try this:

```sh
$ deno run https://deno.land/std/examples/welcome.ts
```

The file will automatically be downloaded (and cached) so it can be run. If you
think that can lead to security problems, think again about permissions you
gave to that script: none. It won't be able to use your filesystem or your
network.

Imports are ES6 imports, and you can also use URLs as we did before:

```js
import * as log from "https://deno.land/std/log/mod.ts";
```

Or specifying a certain version:

```js
import * as log from "https://deno.land/std@0.51.0/log/mod.ts";
```

No more `require()` and differences from the web JavaScript! 
About that: Deno tries to use the already existing standard API instead of
creating new ones. Everything that does not use the `Deno` global namespace
should run just fine on a browser too! (You can think about `fetch()`, for
example)


### Workers

Deno supports Web Worker API. This is a great feature to run CPU intensive
tasks since they will run in a different threads.

One caveat: workers can't use the `Deno` namespace (so accessing Deno specific
features), but this is already implemented as an *unstable* feature.

Note that Node has this feature too, since v10.5.0.

[More details](https://deno.land/manual/runtime/workers)


### Bundling

Bundling is somewhat like compiling a project.

`deno bundle <file> out.budle.js` will output a single JavaScript file
containing all the dependencies.
You can then run this single file with `deno run out.bundle.js` and that's it.

If you also add the fact that `deno` is a self contained binary, you just need
these two files to deploy your app on a different computer.

Side bonus: as long as you don't use the `Deno` namespace, you can import the
bundle in the browser too! Just use:

```html
<script type="module" src="out.bundle.js"></script>
```

[More details](https://deno.land/manual/tools/bundler)


### Formatter

`deno fmt somefile.ts` will beautify your files following some standardize
rules. (Note: the existing `somefile.ts` is overwritten)

This is something you can easily overlook but believe me, an enforced style
guide can be really helpful when you have to read code written by someone else,
since you're already used to it. I **love** `go fmt` and definitely like its
equivalent `deno fmt`.

[More details](https://deno.land/manual/tools/formatter)


## Conclusions

I think Deno is still just at the beginning of its journey. There are great
features, its standard library is still getting worked on while already
providing many functionalities. And yet it's much safer than Node: whit
explicit permission grants and "quit on errors".

Even if my title is provoking, keep an eye on Deno and consider it a good
alternative to Node.js to start writing Typescript easily and without depending
on third-party libraries (i.e. Webpack and Babel).
