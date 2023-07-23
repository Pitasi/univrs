---
title: "Astro: writing static websites like it’s 2023"
datetime: 2023-04-21T16:13:31.683Z
---

_“For fuck sake, another fucking JS framework??”._ I can read your mind! Bear
with me though, I want to give a peek at what Astro has to offer and why Hugo
(or other static site generators) feels obsolete to me.

I really wanted to start writing again, I'm currently reading
_[On Writing Well](https://www.amazon.com/Writing-Well-Classic-Guide-Nonfiction/dp/0060891548)_
and it’s inspiring. I truly believe that getting better at writing has a
positive impact on the communication skills of a person. But as a classic
software developer I fell into the trap of rebuilding my blog from scratch.
Again.

The only positive part is that I have something to blog about. In fact, this
page you are reading is built with Astro! I took a template [made by
TailwindUI](https://tailwindui.com/templates/spotlight) for Next.js and ported
each of the components from React to Astro (well, _almost_ each of them,
continue reading to see what I mean). Here’s my experience with it.

---

Astro is a static site generator. Generating websites is nothing new, here’s a
list of almost 350 generators written in every existing programming language:
[https://jamstack.org/generators/](https://jamstack.org/generators/).

This rise of generators came from a need: solving the pain of having to manually
write HTML files (and maybe upload those with FTP right? _good ‘ol days_).

Why do I call it **pain**?

**Reason 1:** copy-pasting everything: we are good software developers, we stick
to DRY and duplications give us creeps.

<div class="max-w-lg">
  <image webp="https://assets.anto.pt/articles/astro/astro-1.webp" png="https://assets.anto.pt/articles/astro/astro-1.png"></image>
</div>

Thinking of duplicating the navbar in all my HTML pages and trying to
remember to correctly set the _active_ class seems hard enough to me. Only to
suddenly realize I want to add a new link or slightly edit the CSS,
that means editing a lot of HTML files 😱.

**Reason 2:** HTML gives me XML vibes. It’s bloated and it’s incredibly distracting to open `<p>`, then close it with `</p>`. All I want is to write the **content** which is the only thing I should be focusing on, not HTML tags.

Frameworks and markdown language started to shine. ✨

_(You know, for some times we even had clumsy programs that featured a WYSIWYG
editor for HTML, like Adobe Dreamweaver, which by the way, [still exists](https://www.adobe.com/products/dreamweaver.html)!)_

_(Oh and I’m NOT considering CMSs for this post. They can be great but the title
says “2023” and having a PHP+MySQL Wordpress instance is boring, isn’t it?)_

Let’s talk **Hugo**: the most popular open-source static site generator. Its
homepage states: “_Hugo makes building websites fun again_”. Right.

Here’s my experience with Hugo:

- Find a good looking and simple theme from [https://themes.gohugo.io/](https://themes.gohugo.io/)
- Clone it as a submodule in your repo:
  `git submodule add https://github.com/{name} themes/{name}`
- Tweak a little the `config.toml` and start writing some markdown files

Easy peasy: Hugo generates HTML pages ready to served by any HTTP server (you
can use GitHub pages!). Plus, your new site is already SEO friendly, not like
those SPAs that need server side rendering...but (you saw a “but” coming, didn't
you?) customizing the theme quickly becomes cumbersome. Hugo uses the
[Go template syntax](https://gohugo.io/templates/introduction/) which I find to be
unintuitive, and adding any Javascript logic to build dynamic elements it’s not
a great experience either. Almost feels like we are going back to the jQuery
days!

Let me bring you into the future of static site generators with Astro.

## Astro components

A component is a piece of UI that can be reused. Similarly, with Hugo you would
have template files importing other templates to achieve this kind of modularity
and reusability.

In Astro, a component is an `.astro` file and its content is roughly this:

```jsx
---
const visible = true;
const items = ["Dog", "Cat", "Platypus"];
---

<ul>
  {items.map((item) => (
    <li>{item}</li>
  ))}
</ul>

{visible && <p>Show me!</p>}

{visible ? <p>Show me!</p> : <p>Else show me!</p>}
```

In the frontmatter (between `---`) you can put javascript code. _Note_: it is only executed once during build time! This mean you can read from files, make asynchronous API calls, or whatever you need to populate your page.

Outside the frontmatter there is the HTML template enhanced with a jsx-like syntax for interpolating variables.

**Pro:**

- JSX is an user friendly syntax
- VSCode supports JSX and there’s an extension specific for Astro
- Can use any Node.js library (eg. formatting dates, connecting to a database or 3rd party API, …)

**Cons:**

- Components are…static, as expected!
  You can't build a “Counter” or a “To Do list” (but hold on until I tell you about _islands_).

### First class .md/.mdx support

I mentioned Hugo earlier, which is commonly used for blogs since you can write articles as markdown files. Astro supports markdown as well!

Any `.md` (or even `.mdx`) file inside the `/pages` folder will be parsed and converted to a plain JavaScript object. An Astro component called `layout` will receive this object as a variable and can use it in the JS frontmatter:

`article.md`:

```markdown
---
layout: ../layouts/Post.astro
title: Hello, World
author: "Matthew Phillips"
date: "09 Aug 2022"
---

# Hi there!

This is your first markdown page. It probably isn't styled much, although
Markdown does support **bold** and _italics._

To learn more about adding a layout to your page, read the next section on **Markdown Layouts.**
```

`Post.astro`:

```jsx
---
const { frontmatter } = Astro.props;
---
<html>
  <head>
    <title>{frontmatter.title}</title>
  </head>
  <body>
    <h1>{frontmatter.title} by {frontmatter.author}</h1>
    <slot />
    <p>Written on: {frontmatter.date}</p>
  </body>
</html>
```

I find this to be incredibly beautiful in its simplicity. The only thing to notice is that the actual body of your post will end up in the `<slot />` of the component.

### A world without JavaScript

If you read until this point you are ready to use Astro and build a static site. Like Hugo, launching `astro build` will output a folder with plain HTML files ready to be served.

Not a single line of Javascript inside.

That’s better than normal SSR (looking at you Next.js), since it’s only being
built once. Some rustaceans would even call this
[_zero cost abstraction_](https://stackoverflow.com/questions/69178380/what-does-zero-cost-abstraction-mean)!
🦀

---

## Islands

Islands are certainly one of the reason why Astro got its popularity. An island is a single piece of your static webpage that is not actually static.

If you’re wondering why they’re called “_islands_”, it’s a name conied by Etsy’s frontend architect Katie Sylor-Miller in 2019 during a meeting with the creator of Preact Jason Miller. He explained the islands architecture in [this post](https://jasonformat.com/islands-architecture/). In a world where microfrontends exist, islands make sense too!

Let’s consider this page of my blog, its layout is:

<div class="max-w-lg">
  <image webp="https://assets.anto.pt/articles/astro/astro-2.webp" png="https://assets.anto.pt/articles/astro/astro-2.png" alt="Diagram of the layout of this blog"></image>
</div>

Unsurprisingly I have a `Content.astro` and a `Footer.astro` file. I also want to keep the nice effect on the navbar (scrolling up a little makes the navbar reappears) and more importantly the “toggle” button on mobile.

I need some Javascript.

One possibility is to write `Header.astro` like this:

```jsx
<div id="navbar">
	<!-- content -->
</div>

<script>
  // do stuff in client
</script>
```

If you have a keen eye you have noticed the ⚛️ logo in the picture above. That’s because writing vanilla JS wouldn’t respect the title of the post (again!). We are in 2023 and I want to use some cool framework otherwise nobody will consider me a cool kid.

Let’s write a normal React component:

`Header.tsx`

```jsx
const Header = () => {
  // do stuff
  return <div>{/** content **/}</div>;
};

export default Header;
```

And prepare yourself for some Astro magic 🪄, here’s my `Layout.astro`:

```jsx
---
import Header from "../components/Header.tsx";
import Content from "../components/Content.astro";
import Footer from "../components/Footer.astro";
---

<body>
  <Header />
  <Content />
  <Footer />
</body>
```

Did you see that? We just imported and used a React component like it was nothing!

### Hydration

Hold on, my page still has 0KB of JS inside and my header doesn’t work the way it’s supposed to.

Astro lets you decide if your component should be interactive, and _when_. This
process is called **hydration** and runs in the client. There are
[a bunch of strategies](https://docs.astro.build/en/core-concepts/framework-components/#hydrating-interactive-components)
available, the most important to me are:

```jsx
// React will render on the server. No JS sent to the client.
// The component will NOT be interactive.
<Header />

// JS will be downloaded as the page loads.
// The component will become interactive as soon as possible.
<Header client:load />

// JS will be downloaded after user scrolls and component is visible.
// The component will become interactive if and when will appear in the page.
<Header client:visible />

// No server-side rendering. JS will be downloaded as the page loads.
// The component will render and become interactive as soon as possible.
// It won't be present in the original HTML sent by the server.
<Header client:only="react" />
```

Since the Header is going to always be visible as the page loads, I chose `client:load` and the component will be interactive as soon as possible:

```jsx
---
import Header from "../components/Header.tsx";
import Content from "../components/Content.astro";
import Footer from "../components/Footer.astro";
---

<body>
  <Header client:load />
  <Content />
  <Footer />
</body>
```

Now, if you open the network tab of your browser in any page of my blog you’ll see React and my component’s code being downloaded:


<image webp="https://assets.anto.pt/articles/astro/network.webp" png="https://assets.anto.pt/articles/astro/network.png" alt="Network tab showing three .js files being downloaded"></image>

Note: if I'll ever want to rewrite my component in something different than
React I can, and maybe even save some precious KBs. For now, I just
limited myself to re-use the component provided by TailwindUI (originally for
Next.js).

### Limitations and caveats

Being able to specify when a component should be hydrated is a pretty powerful
feature to have. Pages can be rendered statically and defer any JS loading for as long
as possible.

That said, I found an annoying limitation: React components cannot import Astro
components. It makes sense: how could React know how to render an Astro
component? Still I found myself with a static component (an icon) as an Astro
component, and I really wanted to just reuse it inside my React component.

The common denominator is to have it as a React component, both Astro and React
components will be able to use it. As long as you don’t hydrate it, you’re
not adding unneded extra bytes of JS.

---

## Where Astro shines: extensibility

I showed you how Astro generates static sites, from components or markdown
files. And how to use React (or other frameworks) seamlessly inside your Astro
components.
It's time to put it all together.

For a developer, it's really easy to write your own utilities as components. In
this page you might not have noticed that all the diagrams are present in two
flavors: light and dark.

<img
  src="https://assets.anto.pt/articles/astro/light-dark.gif"
  alt="Gif showing that switching from dark to light theme on the site loads a different set of images"
/>

This was made possible by a simple wrapper component I made and used like this:

```js
<Image src="/light-img.png" darkSrc="/dark-img.png" />
```

Its source code can fit in just a few lines:

```js
---
const { darkSrc, ...props } = Astro.props;
const darkProps = { ...props, src: darkSrc };
---

{ darkSrc ? (
	<img class="dark:hidden block" {...props} />
	<img class="dark:block hidden" {...darkProps} />
) : (
	<img {...props} />
) }
```

If I supply a `darkSrc` prop, instead of rendering a single `<img />`, there
will be two of them: one for the light theme (hidden when dark theme is
selected), and one for the dark theme (hidden when light theme is selected).

If I don't supply a `darkSrc` prop, it will just render a single `<img />` as usual.

My point is not really about that simple `<Image />` component but more about
_how easy_ is to jump in and extend your website when needed, even if you are
starting from a template made by someone else.

Since it's so easy to build components, you can share them with the community!
In fact, Astro has a dedicated page: https://astro.build/integrations/. The ones
I'm using for this blog are Tailwind, React and Mdx.

## Deployments

I want to close this article with one of the most annoying part for a software
developer: _"Where do I host my new beautiful blog?"_

Well, fortunately you can host if for free on
[Vercel](https://docs.astro.build/en/guides/deploy/vercel/), [Netlify](https://docs.astro.build/en/guides/deploy/netlify/), or
[GitHub Pages](https://docs.astro.build/en/guides/deploy/github/)!

I'm not covering "how to do it", but follow any of the above links. I promise
you it's not hard. Personally I have chosen Vercel.

---

## Closing thoughts

Anyone who knows me knows that I like to experiment, a lot. I had a great
time making this blog in Astro, that’s why I wanted to share my opinions and
used it for my personal website.

I learnt how to use it in literally minutes and you don’t have to know anything
about React (or any framework) to start building with it. I prefer Astro to Hugo
as I find component-driven development to be feel more “natural” instead of
other templating syntaxes.

At the same time, you can still extend your components with some logic by using
any framework you want. You can even reuse their existing code as-is.
If you find a neat component on the web, you can just import it and use it.
It doesn't matter if it's written for React, Vue, or whatever.

That’s it for today - if this inspired you and want to start playing with
Astro, I suggest their [make a blog](https://docs.astro.build/en/tutorial/0-introduction/) tutorial.
