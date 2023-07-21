---
title: "An eShop price comparison app"
datetime: 2018-02-15T14:12:17.000Z
---

**TL;DR**: [https://esho.pw](https://esho.pw), I made this!

## Intro

Recently I bought a Nintendo Switch. That's by far the best game console I had in a long time, but you can read all about it online, I'm not here to talk about that!

Here's the fact: with Nintendo Switch it's possible to buy games in foreign online shops with only one limitation: you can only buy from a shop in the same _big region_ as you are.

These big regions are:

*   **Region 1**: Americas
*   **Region 2**: Europe + Russia + South Africa
*   **Region 3**: Japan

## How to buy in different eShops

_Switching_ (pun intended) your account region is definitely easy, just log in from here: [https://account.nintendo.com](https://account.nintendo.com) and change your profile settings.  
You can do this everytime you want, **but you can't change region if you have balance left on your account** (so make a secondary account if you want to use foreign gift cards).

**Best thing of this**: some games are cheaper in other shops. Mostly because of currencies exchange rates, but also because devs can adapt their games prices to the local market and lifestyle.

## Why did I do this?

I found a couple of website comparing prices between the different stores.

The first one shows prices in USD only, which may be good for people living in US but not for me in Italy.

The second one is a giant table with a row for every game and every country as a column. It also allows to show prices in your currency, and that's fine.

The problem is that searching games wasn't easy, especially for low powered devices such as a smartphone, and the initial loading delay every time was HUGE.

_(EDIT: The website I'm talking about has been updated since I started working on my own site and writing this article. It now offers a better UI but still no favorites, no saves, and it's pretty slow)_

As a guy who doesn't give up and likes to automate repetitive tasks I though: _"Okay then, I'll do it myself"_.

![A man with his hands covered with mud](https://images.unsplash.com/photo-1483569577148-f14683bed627?ixlib=rb-0.3.5&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=1080&fit=max&ixid=eyJhcHBfaWQiOjExNzczfQ&s=8fae7fa69258ad14730ebd84a2bcc214)  
<small>Not afraid to get my hands dirty. Photo by [jesse orrico](https://unsplash.com/@jessedo81?utm_source=ghost&utm_medium=referral&utm_campaign=api-credit) / [Unsplash](https://unsplash.com/?utm_source=ghost&utm_medium=referral&utm_campaign=api-credit)</small>

## Yay - let's get to work

So here starts my journey made of experimenting new (for me) stuff and trying to achieve a decent result.

### Scraping Nintendo

The first thing I wanted to build was the database. I've been already using **MongoDB** with **Mongoose** in a Node.js environment so I didn't have any second though picking them again. As the Node.js community is very large and very active, there was already a module on NPM for scraping the Nintendo _(internal)_ API.

Using that module I found out that Nintendo has totally different responses for the three different big regions (see above). So after fetching the initial data I had to parse them into an unified object type (why Nintendo isn't doing it themselves it's unknown to me, they are basically having three different web apps to be mantained). Also, while parsing I stored every price converted in USD (using [OpenExchange Rates](https://openexchangerates.org/)).

**To do**: scraping Metacritic for every game score.

That's basically it for the backend.

![](https://images.unsplash.com/photo-1454321717968-d243ade71663?ixlib=rb-0.3.5&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=1080&fit=max&ixid=eyJhcHBfaWQiOjExNzczfQ&s=c91eb27406aa5e4043e868287d6e51ce)  
<small>The backstage. Photo by [seabass creatives](https://unsplash.com/@sebbb?utm_source=ghost&utm_medium=referral&utm_campaign=api-credit) / [Unsplash](https://unsplash.com/?utm_source=ghost&utm_medium=referral&utm_campaign=api-credit)</small>

### Serving the data

I initially set up a classic REST Api with just a GET endpoint for downloading the game list, but then switched to webhooks - this way it was easier to set up a client **synchronization** feature.

I added an endpoint for fetching the current exchange rates and the active shops list, in addition to the games list. I'm thinking about leaving these REST endpoints free for everyone who wants to build their own application.

About that synchronization, I used the new [Telegram login widget](https://telegram.org/blog/login) to let the user link their Telegram account on my website for storing settings and favorites on the server. I also used [JWT](https://jwt.io/) to remember the user for some days at least. This was the first time using both the Telegram login and the JWT tokens, but it was really straightforward - can't wait to use them again on my next project.

### A clean UI but not so clean under the hood

At this point, my project was missing one last important piece: an **user interface**.

I absolute love [React](https://reactjs.org/) and so I used it, but as I was writing my application - it all became _tangled_. I had a lot of pieces of state in different files.

Everyone was passing down props from the router, accessing the browser localStorage. **Things had got out of hand**.

One hero came to rescue me: [Redux](https://redux.js.org) (as a pro-tip for everyone reading this who never used Redux and doesn't know what is this library, I suggest the excellent (free) videos from its creator itself Dan Abramov: [https://egghead.io/courses/getting-started-with-redux](https://egghead.io/courses/getting-started-with-redux)).

After a big refactor, I succesfully inserted Redux inside the app. All the state is persistent thanks to the localStorage, and every component don't update the state - instead they dispatch predefined actions. It all became clear and easily extendable - I'm kinda proud of myself.

I didn't mention it yet, but I used [Bulma](https://bulma.io) as CSS Framework. I think about it as a lightweight, **easier**, and modern Bootstrap.

Add some fancy animation et voilà: **[https://esho.pw](https://esho.pw)**.

![](https://images.unsplash.com/photo-1511872638242-f1652016540e?ixlib=rb-0.3.5&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=1080&fit=max&ixid=eyJhcHBfaWQiOjExNzczfQ&s=4dbef0f3709fa4179f1eddb94da47d22)  
<small>My stack. Photo by [Thought Catalog](https://unsplash.com/@thoughtcatalog?utm_source=ghost&utm_medium=referral&utm_campaign=api-credit) / [Unsplash](https://unsplash.com/?utm_source=ghost&utm_medium=referral&utm_campaign=api-credit)</small>

* * *

_I usually don't write in English, and don't write this kind of post. Still trying to improve myself - any feedback is appreciated._
