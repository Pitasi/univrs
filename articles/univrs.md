---
title: "univrs"
datetime: 2023-07-24T21:02:01.000Z
unlisted: true
---

Instead of drowning in the labyrinth of mainstream frameworks like Next.js, I decided to take a wild turn: create my solution in Rust.

It's still convoluted, but I can lay out the implementation right here in this blog post. Can you say the same for Next.js?

/callout Are you saying that you built your own version of Next.js?
/callout Not at all! That would require an enormous amount of work and I only worked on this in my spare time. I wanted to learn more Rust and played with it building something that I could actually use in real life: a webserver. 

I think of this as the Rust equivalent of [T3 Stack](https://create.t3.gg/). I did not implement a framework, I took existing libraries and put some glue in between them.

## The goal
I know you're eager to dive in, let me show you what it looks like:
- screenshots
- custom comps in markdown

## Old school server-side rendering

Remember the good old days when internet pages were just HTML files hitching a ride over the internet? Yeah, those days. Well, they're not over! We've just made it more intricate than a season finale of a soap opera. If you're a frontend newbie, you might be fooled into believing that Node, JavaScript, React, or similar tools are a necessity to build a website.

Let's begin our adventure:
- excalidraw architecture AXUM --html-> USER

- axum hello world

Now go and build your blog, that's all you need really.

The next step is playing a bit with `format!()` to avoid duplicated HTML, that's often called templating.

- axum hello world 2 pages sharing root layout

I will be calling functions like `layout()` *components*. Because that's all they are if you think about it:

```jsx
// A JSX component
function Nuts({ count }) {
    if (count < 0) {
	return <p>You cannot have negative nuts</p>;
    }
    return <h1>{count} nuts</h1>;
}
```

equivalent to:
```rust
// A Rust Server Component
fn nuts(count: i64) -> String {
    if (count < 0) {
	"<p>You cannot have negative nuts</p>".into()
    } else {
	format!("<h1>{count}</h1>")
    }
}
```

/callout Are you saying that the "Rust Server Components" that baited viewers here are just functions that return string?
/callout Yep, that's correct.
/callout You gotta put some real content now or nobody will trust you ever again.

## Server components
Next.js and React have been pushing for RSC (the real ones, React Server Components).
It's exactly what I did in the previous section, the JSX component is eventually rendered into a HTML string by the server.

/infopanel Dan Abramov made this amazing presentation where he used Internet Explorer to navigate a page built with RSC.

JSX provide a far better DX than `format!()`, but I've found [maud](https://maud.lambda.xyz/), a crate that is pure gold. (Thanks Xe Iaso's site source code for making me discover maud). It's not as good as writing JSX but it's not bad either.

Even if we're adding a new dependency, keep in mind the philosophy of this project: being able to understand what's going on at all times. Something you definitely cannot do with React/Next.js/JSX, transpilers, bundlers. Maud is just our `format!()`.

- TODO: write a component with Maud (pattern matching and loops)

/callout You say that maud is just `format!()`. Why the function now returns `Markup` instead of `String`?
/callout `Markup` is pretty much a `String`, but for convenience I left it. If you put a normal `String` into a Maud component, its content will be escaped. Returning Markup directly is easier for nesting Maud components.

The power of Maud is the ability to have control flows directly inside your template, it's so convenient. More abilities are documented in the [official website](https://maud.lambda.xyz/control-structures.html).

One last thing about Maud: its `Render` trait.

By implementing `Render`, any type can customize the HTML it will produce when rendered by Maud. By default, the standard `Display` trait is used, but by implementing `Render` manually we can override the behaviour.

This comes in handy for building our custom components:
```rust
struct Css(&'static str);

impl Render for Css {
    fn render(&self) -> Markup {
        html! {
			link rel="stylesheet" type="text/css" href=(self.0);
        }
	}
}
```

## Markdown components
We have learned how to define our custom components, so let's build another useful one: a markdown renderer.

For that, I will add a new crate to our tool belt: [comrak](https://docs.rs/comrak/latest/comrak/).

Defining such a component it's trivial once you have comrak: 
```rust
use comrak::{markdown_to_html, ComrakOptions};

pub struct Markdown(pub String);

impl Render for Markdown {
    fn render(&self) -> maud::Markup {
        let options = ComrakOptions {
            ..comrak::ComrakOptions::default()
        };
        let html = markdown_to_html(&self.0, &options);
        maud::PreEscaped(html)
    }
}
```

We can pull together a full webpage easily:
```rust
pub async fn page() -> Markup {
    html! {
        h1 { "Sample Page" }
        (Markdown("
[Click me](https://www.youtube.com/watch?v=dQw4w9WgXcQ).
        ".to_string()))
    }
}
```
Beautiful:
<image png="https://assets.anto.pt/articles/rsc/sample_page.png" webp="https://assets.anto.pt/articles/rsc/sample_page.webp" avif="https://assets.anto.pt/articles/rsc/sample_page.avif"></image>

## MD...X?
MDX allows you to use JSX in your markdown content. And I wanted something similar.

/callout Are we the reason you wanted custom components?
/callout Bingo. Here is our source code: -todo add source code

To achieve that, I'm adding one more crate: [lol-html](https://crates.io/crates/lol-html). Built by CloudFlare to power their Workers.

> _**L**ow **O**utput **L**atency streaming **HTML** rewriter/parser with CSS-selector based API._

What lol-html really is, is a fancy search-and-replace for HTML.

You can search by using CSS selectors, and replace by using a set of API they expose.
First you set up a `rewriter`, then you feed it with your HTML stream. Let's see an example for rewriting all `<a href="http://..."` with a `https` version:

```rust
let mut output = vec![];
let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
		        // match all <a>
                element!("a[href]", |el| {
	                // extract their href
                    let href = el
                        .get_attribute("href")
                        .expect("href was required")
                        // put an s in that http
                        .replace("http:", "https:");

					// replace the href value
                    el.set_attribute("href", &href)?;

                    Ok(())
                })
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c)
    );
    
rewriter.write(b"<div><a href=")?;
rewriter.write(b"http://example.com>")?;
rewriter.write(b"</a></div>")?;
rewriter.end()?;

assert_eq!(
	String::from_utf8(output)?,
	r#"<div><a href="https://example.com"></a></div>"#
);
```

We can build a generalized version that wraps a component applying the provided Settings:
```rust
use maud::{Markup, PreEscaped};

pub fn apply<'s, 'h>(settings: Settings<'s, 'h>, html: Markup) -> Markup {
    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(settings, |c: &[u8]| output.extend_from_slice(c));
    rewriter.write(html.0.as_bytes()).unwrap();
    rewriter.end().unwrap();
    PreEscaped(String::from_utf8(output).unwrap())
}
```

Now we can enhance our Markdown component, with a little juggling between types:
```rust

pub struct EnhancedMd(pub Markdown);

impl Render for EnhancedMd {
    fn render(&self) -> Markup {
        apply(
            Settings {
				// define an <alert msg=xxx> component
                element_content_handlers: vec![element!("alert", |el| {
                    let msg = el.get_attribute("msg").expect("msg attribute is required");
                    el.replace_comp(html! {
                        div style="background: red; padding: 10px;" { (msg) }
                    });
                    Ok(())
                })],
                ..Settings::default()
            },
            html! {
                (&self.0)
            },
        )
    }
}

// add replace_comp()() to lol-html's Elements so that we can pass Markup components directly directly
pub trait ComponentReplacer {
    fn replace_comp(&mut self, comp: Markup);
}

impl ComponentReplacer for Element<'_, '_> {
    fn replace_comp(&mut self, comp: Markup) {
        self.replace(&comp.0, ContentType::Html);
    }
}
```

And use it:
```rust
pub async fn page() -> Markup {
    html! {
        h1 { "Sample Page" }
        (EnhancedMd::from(r#"
[Click me](https://www.youtube.com/watch?v=dQw4w9WgXcQ).
<alert msg="you should really click that link"></alert>
        "#.to_string()))
    }
}
```
<image png="https://assets.anto.pt/articles/rsc/enhanced_sample_page.png" webp="https://assets.anto.pt/articles/rsc/enhanced_sample_page.webp" avif="https://assets.anto.pt/articles/rsc/enhanced_sample_page.avif"></image>

## Going interactive
```rust
use maud::{html, Markup};

static mut COUNTER: u32 = 0;

pub fn counter() -> Markup {
    // do some heavy db query here
    let c = unsafe { COUNTER };
    html! {
        div {
            p { "Counter:" (c) }
            button { "Increment" }
        }
    }
}

pub async fn page() -> Markup {
    html! {
        h1 { "Sample Page" }
        (counter())
    }
}

```

<image png="https://assets.anto.pt/articles/rsc/sample_page_counter.png" webp="https://assets.anto.pt/articles/rsc/sample_page_counter.webp" avif="https://assets.anto.pt/articles/rsc/sample_page_counter.avif"></image>

Clicking the button doesn't do anything, yet.

We'll be using a JavaScript library that recently gained a lot of popularity, as it allows to not write any JavaScript: [htmx](https://htmx.org/).

The idea is that when the user clicks on the button, it will fire a `POST /components/counter/increment` request to our server, that will update the counter and reply with the updated HTML of that component.

```rust
// register new routes specific to this component to the axum router
pub fn register(router: Router) -> Router {
    router.route("/components/counter/increment", post(counter_increment))
}

static mut COUNTER: u32 = 0;

// handle POST requests
pub async fn counter_increment() -> Markup {
    unsafe { COUNTER += 1 };
    counter()
}

pub fn counter() -> Markup {
    // do some heavy db query here
    let c = unsafe { COUNTER };
    html! {
        div {
            p { "Counter: " (c) }
            button { "Increment" }
        }
    }
}

pub async fn page() -> Markup {
    html! {
        h1 { "Sample Page" }
        (counter())
    }
}
```

We can easily test our new endpoint with `curl`:
```sh
$ curl -XPOST http://localhost:3000/components/counter/increment
<div><p>Counter: 1</p><button>Increment</button></div>

$ curl -XPOST http://localhost:3000/components/counter/increment
<div><p>Counter: 2</p><button>Increment</button></div>
```

Sweet. Now let's make the button call the endpoint and swap its content:
```rust
pub fn counter() -> Markup {
    // do some heavy db query here
    let c = unsafe { COUNTER };
    html! {
        div {
            p { "Counter: " (c) }
            button
                // specifying the element to be replaced and the request
                // to make it's all we need
                hx-target="closest div"
                hx-post="/components/counter/increment"
                { "Increment" }
        }
    }
}

pub async fn page() -> Markup {
    html! {
        // add htmx from a CDN
        script src="https://unpkg.com/htmx.org@1.9.3" {}
        h1 { "Sample Page" }
        (counter())
    }
}
```

And it works:
<image png="https://assets.anto.pt/articles/rsc/sample_page_counter_working.png" webp="https://assets.anto.pt/articles/rsc/sample_page_counter_working.webp" avif="https://assets.anto.pt/articles/rsc/sample_page_counter_working.avif"></image>

This is not an htmx tutorial, I just wanted to showcase how convenient can be to share the same function `counter()` both as a "regular page" that as an endpoint for HTMX.

It's also useful for where I'm going next, suspense...

## Building <Suspense />
We covered a lot, and I must say I was already incredibly happy with all I learned and built along the way. I was so engaged with this pet project that I didn't want to stop there though.

React supports a component called `<Suspense />`, which is typically used for showing a fallback component (e.g. a loading state) while the real component is still rendering (on the server).

In our system that could mean that I don't want to block the render of my page while performing a database query for example.

Think of our previous `counter()` component, imagine if fetching that number from a 3rd party service takes 500ms. If we don't care about the SEO, we can avoid blocking the entire page and let the client fetch the `counter()` component lazily.

- todo excalidraw timeline diagram

By having htmx in place, doing that it's not hard. First I'm going to register a new `GET /components/counter` route that will just return the counter component:
```rust
pub fn register(router: Router) -> Router {
    router
        .route("/components/counter", get(counter_get))
        .route("/components/counter/increment", post(counter_increment))

}

pub async fn counter_get() -> Markup {
    counter()
}

```

And since we don't want to render `counter()`, let's fix the page template:
```rust
pub async fn page() -> Markup {
    html! {
        script src="https://unpkg.com/htmx.org@1.9.3" {}
        h1 { "Sample Page" }
        // this div will be replaced as soon as the page loads
        div hx-trigger="load" hx-get="/components/counter" {
            p { "Counter: loading" }
        }
    }
}
```

Try adding a `sleep()` inside the `counter()` function and you'll see that the rest of the page (i.e. the Sample Page title) will render anyway.

And if you want you can build your own `suspense()` component like this:
```rust
pub fn lazy(route: &str, placeholder: Markup) -> Markup {
    html! {
        // this div will be replaced as soon as the page loads
        div hx-trigger="load" hx-get=(route) {
            (placeholder)
        }
    }
}
```

## Future work

What I showed here is a proof of concept. I'll keep using these libraries and see how it goes, but since I'm not committing to a framework I think I'll be able to build my custom components as I wish.

To end with a pretty example of this, here's how I'm automatically selecting the best possible image format to serve (e.g. AVIF, WEBP, JPG, ...):
```rust
// path is something like "dir/picture.jpg"
pub fn static_img(path: &str, alt: &str, class: &str) -> Markup {
	// `search_available_sources` will access filesystem to find
	// other variants: dir/picture.avif, dir/picture.webp, ...
	// They will be sorted by their actual size on the filesystem.
    let sources = search_available_sources(path);
    if sources.is_empty() {
        panic!("couldn't find any image source for {}", path);
    }

    let (fallback, sources) = sources.split_last().unwrap();

    html! {
        picture class="contents" {
	        // best formats first
            @for source in sources {
                source srcset=(format!("/{}", source.path())) type=(source.mime_type());
            }

			// fallback
            img src=(format!("/{}", fallback.path())) class=(class) alt=(alt) loading="lazy" decoding="async";
        }
    }
}
```

If you enjoyed this article you can find me on Mastodon: [@zaphodias@hachyderm.io](https://hachyderm.io/@zaphodias). Any constructive critique is much appreciated!

Cheers.

