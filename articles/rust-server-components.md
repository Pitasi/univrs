---
title: "Rust Server Components"
datetime: 2023-07-24T21:02:01.000Z
unlisted: true
---

Instead of drowning in the labyrinth of mainstream frameworks like Next.js, I
decided to take a wild turn: create my solution in Rust.

It's still convoluted, but I can lay out the implementation right here in this
blog post. Can you say the same for Next.js?

<dialog
    character=raisehand
    pos=left
    msg="Are you saying that you built your own version of Next.js?">
</dialog>
<dialog
    character=bulb
    pos=right
    msg="
        Not at all!
        That would require an enormous amount of work and I only worked on this
        in my spare time. I wanted to learn more about Rust and played with it
        building something that I could actually use in real life: a webserver.
    ">
</dialog>

I think of this as the Rust equivalent of [T3 Stack](https://create.t3.gg/). I
did not implement a framework, I took existing libraries and put some glue in
between them.

## The goal

I know you're eager to dive in, so let me show you what it looks like.

This very blog page is rendered by a "Rust Server Component":

<image avif="https://assets.anto.pt/articles/rsc/showcase.avif"
    webp="https://assets.anto.pt/articles/rsc/showcase.webp"
    png="https://assets.anto.pt/articles/rsc/showcase.png"></image>

While the actual content of this article is written in Markdown, with a few
custom components I defined:

<image avif="https://assets.anto.pt/articles/rsc/showcase_md.avif"
    webp="https://assets.anto.pt/articles/rsc/showcase_md.webp"
    png="https://assets.anto.pt/articles/rsc/showcase_md.png"></image>

I'm pretty excited with the result I achieved, so let's go!


## Old school server-side rendering

Remember the good old days when internet pages were just HTML files hitching a
ride over the internet? Yeah, those days. Well, they're not over! We've just
made it more intricate than a season finale of a soap opera. If you're a
frontend newbie, you might be fooled into believing that Node, JavaScript,
React, or similar tools are a necessity to build a website.

Let's begin our adventure:

<image avif="https://assets.anto.pt/articles/rsc/exc1.avif" webp="https://assets.anto.pt/articles/rsc/exc1.webp" png="https://assets.anto.pt/articles/rsc/exc1.png" ></image>

```rust
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> impl IntoResponse {
    Html("<h1>Hello</h1>")
}
```

Before building a simple website like a blog, the next natural step is playing
a bit with `format!()` to avoid duplicated HTML:

```rust
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/page1", get(page1))
        .route("/page2", get(page2));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn layout(content: String) -> String {
    format!("
<h1>A fancy website title</h1>
{content}
    ")
}

async fn page1() -> impl IntoResponse {
    Html(layout("<h1>Page 1</h1>"))
}

async fn page2() -> impl IntoResponse {
    Html(layout("<h1>Page 2!!!</h1>"))
}
```

I will be calling functions like `layout()` *components*. Because that's all
they are if you think about it:

```javascriptx
// A JSX component
function Nuts({ count }) {
    if (count < 0) {
        return <p>You cannot have negative nuts</p>;
    }
    return <h1>{count} nuts</h1>;
}
```

is equivalent to:

```rust
// A Rust Server Component
fn nuts(count: i64) -> String {
    if (count < 0) {
        "<p>You cannot have negative nuts</p>".into()
    } else {
        format!("<h1>{count} nuts</h1>")
    }
}
```

<dialog
    character=raisehand
    pos=left
    msg="
        Are you saying that &quot;Rust Server Components&quot; that baited
        viewers here are just functions that return strings?
    ">
</dialog>
<dialog
    character=bulb
    pos=right
    msg="Yep, that's correct.">
</dialog>
<dialog
    character=facepalm
    pos=left
    msg="
        You gotta put some real content now or nobody will trust you ever
        again.
    ">
</dialog>

## Server components
Next.js and React have been pushing for RSC (the real ones, React Server
Components) lately.
It's exactly what I did in the previous section, the JSX component is
eventually rendered into a HTML string by the server.

<dialog
    character=finger
    pos=right
    msg="
        Dan Abramov made
        <a href='https://youtu.be/zMf_xeGPn6s'>this amazing presentation</a>
        where he used Internet Explorer to navigate a page built with RSC.
    ">
</dialog>

JSX provide a far better DX than `format!()`, but I've found
[maud](https://maud.lambda.xyz/), a crate that is pure gold. (Thanks Xe Iaso's
[site](https://xeiaso.net) [source code](https://github.com/xe/site) for making
me discover maud). It's not as good as writing JSX but it's not bad either.

The great thing is that you can't make mistakes like forgetting to close a
`<p>` or your code won't even compile.

Since we are adding a new dependency, I want to keep in mind the philosophy of
this project: being able to understand what's going on at all times.

Maud is just a fancier `format!()` that looks like that:

```rust
// A Rust Server Component with maud
fn nuts(count: i64) -> Markup {
    // You can still put some logic here, if you want.
    // However, maud templating conveniently supports if,
    // let, match, and loops.

    html! {
        @if (count < 0) {
            p { "You cannot have negative nuts" }
        } else {
            h1 { (count) " nuts" }
        }
    }
}
```

<dialog
    character=raisehand
    pos=left
    msg="
        If maud is just a fancier <code>format!()</code>, why the function now
        returns <code>Markup</code> instead of <code>String</code>?
    ">
</dialog>
<dialog
    parse=markdown
    character=bulb
    pos=right
    msg="
`Markup` is a `String`, but it's also a way to express *a string that contains
HTML*. By default maud will escape strings contents. Returning `Markup`
directly is easier for nesting Maud components.
    ">
</dialog>

The power of Maud is the ability to have control flows like if-s and loops
directly inside your template. More features are documented in the [official
website](https://maud.lambda.xyz/control-structures.html).

One last thing about Maud: its `Render` trait.

By implementing `Render`, any type can customize the HTML it will produce when
rendered by Maud. By default, the standard `Display` trait is used, but by
implementing `Render` manually we can override the behaviour.

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

We have learned how to define our custom components, so let's build another
useful one: a markdown renderer.

For that, I will add a new crate to our tool belt:
[comrak](https://docs.rs/comrak/latest/comrak/).

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

And now we can pull together a full webpage easily:

```rust
pub async fn page() -> Markup {
    // load content from a file, a database, ...
    let content = "[Click me](https://www.youtube.com/watch?v=dQw4w9WgXcQ).".to_string();

    html! {
        h1 { "Sample Page" }
        (Markdown(content))
    }
}
```

Check out this beautiful result, appreciating what we achieved so far:

<image png="https://assets.anto.pt/articles/rsc/sample_page.png" webp="https://assets.anto.pt/articles/rsc/sample_page.webp" avif="https://assets.anto.pt/articles/rsc/sample_page.avif"></image>

## MD...X?
MDX allows you to use JSX in your markdown content. I wanted something similar
for this blog.

<dialog
    character=raisehand
    pos=left
    msg="Are we the reason you wanted custom components?">
</dialog>
<dialog
    character=bulb
    pos=right
    msg="
        Bingo. Here is our source code:
        <pre><code>&lt;dialog character=raisehand pos=left msg=&quot;Are we the reason you wanted custom components?&quot;&gt;&lt;/dialog&gt;
&lt;dialog character=bulb pos=right msg=&quot;Bingo. Here is our source code: &lt;pre&gt;&lt;code&gt;stack overflow&lt;/code&gt;&lt;/pre&gt;&quot;&gt;&lt;/dialog&gt;</code></pre>">
</dialog>

To achieve that, I'm adding one more crate:
[lol-html](https://crates.io/crates/lol-html). Built by CloudFlare to power
their Workers:

> _**L**ow **O**utput **L**atency streaming **HTML** rewriter/parser with
> CSS-selector based API._

What lol-html really is, is a fancy search-and-replace for HTML.

You can search by using CSS selectors, and replace by using a set of API they
expose.

First, you set up a `rewriter`, then you feed it with your HTML stream.

Let's see an example for rewriting all `<a href="http://..."` with a `https`
version:

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

// lol-html is built from streaming content really, so you can feed chunks to
// the rewriter
rewriter.write(b"<div><a href=")?;
rewriter.write(b"http://example.com>")?;
rewriter.write(b"</a></div>")?;
rewriter.end()?;

assert_eq!(
    String::from_utf8(output)?,
    r#"<div><a href="https://example.com"></a></div>"#
);
```

We can build a generalized version that wraps a component applying the provided
Settings:

```rust
use maud::{Markup, PreEscaped};

pub fn apply_rewriter<'s, 'h>(settings: Settings<'s, 'h>, html: Markup) -> Markup {
    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(settings, |c: &[u8]| output.extend_from_slice(c));
    rewriter.write(html.0.as_bytes()).unwrap();
    rewriter.end().unwrap();
    PreEscaped(String::from_utf8(output).unwrap())
}
```

And use it to enhance out Markdown component by wrapping it and calling
`apply_rewriter()` after the markdown content has been rendered:
```rust

pub struct EnhancedMd(pub Markdown);

impl Render for EnhancedMd {
    fn render(&self) -> Markup {
        apply_rewriter(
            Settings {
                // define an <alert msg=xxx> component
                element_content_handlers: vec![
                    // match <alert>
                    element!("alert", |el| {
                        let msg = el.get_attribute("msg")
                            .expect("msg attribute is required");

                        // replace the entire <alert> tag with our
                        // maud component
                        el.replace_comp(html! {
                            div style="background: red; padding: 10px;" {
                                (msg)
                            }
                        });
                        Ok(())
                    })
                ],
                ..Settings::default()
            },

            // apply the rewriter to the Markdown component itself
            html! {
                (&self.0)
            },
        )
    }
}

// add a convenient replace_comp() to lol-html's Elements so that we can pass
// Markup components directly instead of wrestling with Strings
pub trait ComponentReplacer {
    fn replace_comp(&mut self, comp: Markup);
}

impl ComponentReplacer for Element<'_, '_> {
    fn replace_comp(&mut self, comp: Markup) {
        self.replace(&comp.0, ContentType::Html);
    }
}
```

I hope I didn't lose you into implementation details, the usage is much
cleaner:

```rust
pub async fn page() -> Markup {
    let content = r#"
[Click me](https://www.youtube.com/watch?v=dQw4w9WgXcQ).
<alert msg="you should really click that link"></alert>
        "#.to_string()

    html! {
        h1 { "Sample Page" }
        (EnhancedMd(Markdown(content)))
    }
}
```

<image png="https://assets.anto.pt/articles/rsc/enhanced_sample_page.png" webp="https://assets.anto.pt/articles/rsc/enhanced_sample_page.webp" avif="https://assets.anto.pt/articles/rsc/enhanced_sample_page.avif"></image>

<dialog
    parse="markdown"
    pos=right
    character=finger
    msg="
Unfortunately due to the streaming-oriented nature of lol-html, I was
not able to use it to replace components' inner content. You can build a
component like that: `<alert msg=foo></alert>`; but not `<alert>foo</alert>`.
    ">
</dialog>

## Going interactive

It's all good and easy to write some static-ish content. The most dynamic thing
you can do now is to load your markdown content from a file or from a database.

What about some real interactivity for your users though? For my website I
wanted that `<3` button you can see in the top-right corner!
(BTW, if you're enjoying this article, this could be the perfect moment to
click it! A login is required though.)

I'm going to start by building a skeleton of something interactive. Like most
frameworks do as a demo, here's a little counter:

```rust
use maud::{html, Markup};

static mut COUNTER: u32 = 0;

pub fn counter() -> Markup {
    // fetch data from a real db instead of accessing global state
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

We'll be using a JavaScript library that recently gained a lot of popularity:
[htmx](https://htmx.org/). With htmx, we'll make our component interactive by
writing the correct amount of JavaScript: zero.

The idea is that when the user clicks on the button, it will fire a `POST
/components/counter/increment` request to our server, that will update the
counter in its global state and reply with the updated HTML of that component.

Let's register a new `counter_increment()` route handler in our Axum's router.
For the response, we can reuse the function `counter()` we defined earlier.

```rust
// register new routes specific to this component to the axum router
pub fn register(router: Router) -> Router {
    router.route("/components/counter/increment", post(counter_increment))
}

static mut COUNTER: u32 = 0;

pub async fn counter_increment() -> Markup {
    // update state
    unsafe { COUNTER += 1 };

    // return updated HTML
    counter()
}

pub fn counter() -> Markup {
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

Sweet. Now to add interactivity, let's add the "on click" event that will fire
the same HTTP request and swap its content with the response:

```rust
pub fn counter() -> Markup {
    // do some heavy db query here
    let c = unsafe { COUNTER };
    html! {
        div {
            p { "Counter: " (c) }
            button
                // target: element that will be replaced
                hx-target="closest div"

                // method and url of the request
                hx-post="/components/counter/increment"

                // since it's a button, htmx by default will set the trigger
                // to be on click

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

And just like that, it works:

<image png="https://assets.anto.pt/articles/rsc/sample_page_counter_working.png" webp="https://assets.anto.pt/articles/rsc/sample_page_counter_working.webp" avif="https://assets.anto.pt/articles/rsc/sample_page_counter_working.avif"></image>

This is not an htmx tutorial, I just wanted to showcase how convenient can be
to share the same function `counter()` both as a "regular page" that as an
endpoint for htmx.

We also set the ground for where I'm going next, suspense...

## Building `<Suspense />`

I could stop right there and I'd be already satisfied with the things I learned
about Rust, Axum, and all the libraries I used. It was so refreshing to things
differently that I didn't want to stop though.

React provides a component called `<Suspense />`, which is typically used for
showing a fallback component (e.g. a loading state) while the real component is
still rendering on the server.

In my blog, I didn't want to block the entire page rendering while waiting for
the database query that returns the number of `<3`. It's not something that I
care about for the SEO anyway so it can safely be deferred for later.

Think of our previous `counter()` component, imagine if fetching that number
from a 3rd party service takes 500ms. We should let the client fetch the
`counter()` component lazily, after the important content.

<image avif="https://assets.anto.pt/articles/rsc/exc2.avif"
    webp="https://assets.anto.pt/articles/rsc/exc2.webp"
    png="https://assets.anto.pt/articles/rsc/exc2.png"></image>

We can leverage htmx, and it's a lot easier than you might think. First I'm
going to register a new `GET /components/counter` route that will just return
the counter component:

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

And since we don't want to render `counter()` anymore, let's replace it with a
placeholder div that will trigger the GET request as soon as the page is ready:

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

At this point you can try adding a `sleep()` inside the `counter()` function
and you'll see that the rest of the page (i.e. the title and the loading text)
will render straight away and won't be blocked by your sleep.

To keep things clean and nice, write your own `suspense()` component!

```rust
pub fn suspense(route: &str, placeholder: Markup) -> Markup {
    html! {
        // this div will be replaced as soon as the page loads
        div hx-trigger="load" hx-get=(route) {
            (placeholder)
        }
    }
}
```

## Future work

I didn't want to go too deep into implementation details. I wanted to focus on
my idea, my proof of concept.

My personal website is now built this way and its source code is available at:
https://github.com/Pitasi/univrs.

I'm not releasing this as a "library" or "framework". If enough people are
interested, we can build boilerplates repositories or components as a starting
point. It's important to me that anything I used can be swapped easily.

The nicest thing about this approach, was the freedom to write any kind of
logic I wanted. To end with an example, here's how I'm
automatically selecting the best possible image format to serve (e.g. AVIF,
WEBP, JPG, ...) based on the files available in the folder:

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
                source
                    srcset=(format!("/{}", source.path()))
                    type=(source.mime_type());
            }

            // fallback
            img
                src=(format!("/{}", fallback.path()))
                class=(class)
                alt=(alt)
                loading="lazy"
                decoding="async";
        }
    }
}
```

---

If you enjoyed this article you can find me on Mastodon:
[@zaphodias@hachyderm.io](https://hachyderm.io/@zaphodias) or
[LinkedIn](https://linkedin.com/in/pitasi).

I hardly take anything personally and appreciate any constructive critique that
makes me learn something new!

Cheers 🖖

