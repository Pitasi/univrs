---
title: "The problem of SSR frameworks"
datetime: 2021-08-13T09:23:32.000Z
---

## (a rant about Next.js and Sveltekit)

Modern frontend development is basically about:

1. Making a good looking UI (i.e. writing css and other javascript tricks to do fancy animations)
2. Fetching data from some API (no matter if you use graphql or plain http requests, you still need to get some data asynchronously)
3. State management across the app (also including caching things, real time events, ...)

Over the time we _(the developers)_ realized that **JavaScript libraries/frameworks are needed** to be efficient when building larger apps. That's how the rise of Angular, React, Vue, and now Svelte, began.

The performance are slightly inferior to the vanilla js but that's ok since we are at a higher level of abstraction from the DOM and we can finally write **good frontend software**.

So, to achieve the three points I was talking about, we have been told that a **[component driven development](https://www.componentdriven.org/)** approach makes it easier. I agree.

We also have been told that server side rendering is necessary if you want to do some SEO and to improve FCP ([first contentful paint](https://developer.mozilla.org/en-US/docs/Glossary/First_contentful_paint), which is [part of SEO](https://developers.google.com/search/docs/advanced/experience/page-experience) now).
Great! **Next.js** and other frameworks like **Nuxt.js** and **Sveltekit** make SSR incredibly simple. Without much friction, all you have to do it's writing a SPA like before.
They also make possible to do other cool stuff like [SSG](https://blog.logrocket.com/ssg-vs-ssr-in-next-js/). I'm amazed.

## So what's the problem with that?

> 2. Data fetching from some API

In order to do server side rendering, both the server (for the first rendering) and the client (after the app hydration) will need to be able to call your API and fetch the data they need on the page. We'll talk about this soon.

As a software engineer I have a fetish for "clean code" ([just don't overuse it](https://overreacted.io/goodbye-clean-code/)).
When I write a UI component I'd like it to be reusable, and to be in charge of only one thing (single responsability principle, right?).

Let me start with an example:

```jsx
// MovieCard.jsx
const MovieCard = ({ title, poster }) => (
  <article>
    <img src={poster} />
    <p>{title}</p>
  </article>
);

export default MovieCard;
```

It's great because it's stateless, it's pure UI. You may argue that it's an incredibly simple component but that's all we're talking about. Keeping things small and simple.

I can easily write [stories](https://storybook.js.org/) for it and handle edge cases like a missing `poster` to show a greyish fallback.

However...where this data comes from? Well, from [TMDB API](https://developers.themoviedb.org/3)! Let's have a look at how we can extend our component library:

```js
// useTMDBMovie.js

const useTMDBMovie = (movieId) => {
  const [isLoading, setIsLoading] = useState(true);
  const [movie, setMovie] = useState(null);

  useEffect(async () => {
    const res = await fetch(`https://api.themoviedb.org/3/movie/${movieId}`);
    const data = await res.json();
    setIsLoading(false);
    setMovie(data);
  }, [movieId]);

  return { movie, isLoading };
};

export default useTMDBMovie;
```

```jsx
// TMDBMovieCard.jsx

const TMDBMovieCard = ({ movieId }) => {
  const { movie, isLoading } = useTMDBMovie(movieId);
  if (loading) {
    return <p>loading</p>;
  }

  const poster = movie.poster_path
    ? `https://image.tmdb.org/t/p/w220_and_h330_face/${movie.poster_path}`
    : undefined;
  return <MovieCard title={movie.title} poster={poster} />;
};

export default TMDBMovieCard;
```

In my opinion (and please correct me if I'm wrong), this is the correct separation of concerns. We have the UI component, the "data fetcher" component, and some sort of "adapter component" in the middle.

We don't depend on TMDB API directly, and it's easy to stub things around without making any actual API request. I could add more providers to support other movie APIs without changing the main MovieCard component.

> **Note**: that's probably overkill for smaller apps and for this example, but I'm thinking big here and I'm sure you get the point :)

Now, let me introduce the homepage:

```jsx
// Homepage.jsx

const Homepage = () => {
  const movieIds = [3, 10, 999];

  return (
    <>
      {movieIds.map((id) => (
        <TMDBMovieCard movieId={id} key={id} />
      ))}
    </>
  );
};

export default Homepage;
```

(For the sake of brevity the list of IDs is hardcoded and not fetched by another http call. I also won't introduce a MovieCardList component for that reason)

Uh-oh. While this may work in Next.js without much trouble, it won't benefit from SSR at all!
The useTMDBMovie hook is executed, but it immediately returns a loading state so the page rendered on the server will only contain a bunch of "loading" boxes.

Why is that? Because Next.js forces you to use their [data fetching methods](https://nextjs.org/docs/basic-features/data-fetching) (e.g. getStaticProps) to generate props for the Homepage component server-side.

Only inside those methods you can perform async operations that will be waited for rendering the page. And this methods only apply to "page" component, you can't use them directly inside TMDBMovieCard!

The exact same thing happens in Sveltekit and it's [load function](https://kit.svelte.dev/docs#loading).

If you want to truly server side render the list of movies, you have to load all the things before React can render any element.

So you might end up with something like this:

```jsx
// Homepage.jsx

const Homepage = ({ movies }) => (
  <div>
    {movies.map((id) => (
      <MovieCard key={id} />
    ))}
  </div>
);

Homepage.getServerSideProps = async () => {
  const movieIds = [3, 10, 999];
  const movies = await Promise.all(
    movieIds.map((id) =>
      fetch(`https://api.themoviedb.org/3/movie/${id}`).then((res) =>
        res.json()
      )
    )
  );

  return {
    props: { movies },
  };
};
```

Notice how **we got rid of TMBDMovieCard entirely**. That's because now our Homepage has to know how to fetch data from TMDB and extract the correct attributes from the movies objects (in other terms: our Homepage depends on TMDB).

I could refactor the previous TMDBMovieCard to take a movie as a prop instead of fetching it, but it sounded like more overhead than benefit.

If we want to support movies from different providers other than TMDB in the future we will have to change our homepage implementation. You may be able to clean up the code a bit using a global state or context and some custom React hooks. Anyway, you still are going to populate that state from the Homepage component and consuming it from inner children.
That's kinda awkward.

So here's a thought: **what if components could do some data fetching, and still be rendered on the server?**

Let me introduce you to [Nuxt.js fetch hook](https://nuxtjs.org/docs/2.x/components-glossary/pages-fetch):

## Nuxt.js fetch hook

When I tried Nuxt (and Vue) for the first time a couple days ago, I was astonished by the `fetch` hook. It precisely solves the problem described in this blog post.

Let me rewrite my components:

```js
// Homepage.vue

<template>
  <div>
    <TMDBMovieCard
      v-for="id in movieIds"
      v-bind:key="id"
      v-bind:movieId="{ id }"
    />
  </div>
</template>

<script lang="ts">
import Vue from "vue";
export default Vue.extend({
  data: () => {
    movieIds: [3, 10, 999],
  },
});
</script>
```

```js
// MovieCard.vue

<template>
  <article>
    <img src="{poster}" />
    <p>{title}</p>
  </article>
</template>

<script lang="ts">
import Vue from "vue";
export default Vue.extend({
  props: ["title", "poster"],
});
</script>
```

```js
// TMDBMovieCard.vue

<template>
  <p v-if="$fetchState.pending">Fetching movie...</p>
  <p v-else-if="$fetchState.error">An error occurred :(</p>
  <MovieCard v-else poster="{poster}" title="{title}" />
</template>

<script lang="ts">
import Vue from "vue";
export default Vue.extend({
  props: ["movieId"],
  data: () => {
    return {
      movie: null,
    };
  },
  async fetch() {
    this.movie = await this.$http.$get(
      `https://api.themoviedb.org/3/movie/${this.proposal.movieId}`
    );
  },
  computed: {
    poster: () => {
      if (!movie?.poster_path) {
        return undefined;
      }
      return `https://image.tmdb.org/t/p/w220_and_h330_face/${movie.poster_path}`;
    },
    title: () => movie?.title,
  },
});
</script>
```

And that's basically it. I cheated a little though, I joined the `useTMDBMovie` hook and the `TMDBMovieCard.jsx` into a single `TMDBMovieCard.vue` component.

I'm not a Vue expert, I just skimmed really fast through their docs and written a basic Nuxt.js app, certainly there are ways to separate the data fetching in a separate and re-usable function.


## A skeleton in the closet

In 2021, we are definitely used to seeing things loading. But we also want users to not "feel" things loading.

This may sound like a nonsense but psychology helped a little in cheating users' perception: the most popular trick is to "show something" while the page or the components are loading. It may be a combination of some placeholder text and gray skeleton with fade or wave gradient animations.

The goal is to keep the user busy trying to understand what's going on and how the page *will* look like for that 100ms (if you're lucky) while your app is fetching data.

One of the main advantages of SSR is that you send the final HTML in the page request. It doesn't makes much sense to send HTML skeletons, it's way better to send the real content.
After that initial rendering, navigation inside your hydrated app should be fast if you want to impress your users. That's why Next.js implemented magic tricks for pre-downloading the pages where the user may go. (E.g. when hovering a link with the mouse, that page gets downloaded in the background without the user noticing. When user clicks, the page is served from disk and feels incredibly snappy).

Still, what if this preloading is not sufficient? If the users clicks fast enough that the page hasn't finished loading yet? I want to show skeletons...but I can't!

The last time I tried, both in Next.js that Sveltekit was a real pain. Here's a workaround I came with using Sveltekit's layout:

```svelte
// _layout.svelte

<script lang="ts">
  import Skeleton from '$lib/Skeletons/index.svelte';
  import { navigating } from '$app/stores';
  import type { Navigating } from '$app/stores';
  import { derived } from 'svelte/store';
  const delayedNavigating = derived(navigating, (currentNavigating, set) => {
    setTimeout(() => set(currentNavigating), 150);
  });
  let currentNavigating: Navigating;
  navigating.subscribe((val) => (currentNavigating = val));
</script>

<div>
  <main role="main">
    {#if $navigating && $delayedNavigating}
      <Skeleton page={currentNavigating.to} />
    {:else}
      <slot />
    {/if}
  </main>
</div>
````

```svelte
// skeletons.svelte

<script lang="ts">
  import type { Page } from '$app/stores';
  export let page: Page = null;
</script>

{#if page.path.startsWith('/list')}
  <div>
    <p>skeleton for /list</p>
  </div>
{:else}
  <p>other skeletons...</p>
{/if}
````


Basically I have to intercept every route change event, and display a custom skeleton page while the real page it's loading.

It's kinda annoying because I have to mantain two versions of pages. I'd rather have each single component defining a placeholder when it makes sense.

If there are better ways for handling that, I still haven't found them.


## Conclusions

This post is just a rant, like the title says. I still think that Next.js, Sveltekit, Nuxt.js, and SSR in general are great and worth to be used instead of "classic" SPAs.

I'm not saying that Nuxt is better than Next, I just pointed out a lack of a feature that I personally would find very useful. However, workaround exists and this may not be a problem for you.
