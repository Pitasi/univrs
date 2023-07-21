---
title: "Cross-platform MacOS Mojave-like wallpapers"
datetime: 2018-08-16T14:42:24.000Z
---

<img alt="Animation of wallpaper changing sun position during the day" src="https://camo.githubusercontent.com/f6acc497624cf90f9b4debf275845c5dfdb78af4/68747470733a2f2f692e696d6775722e636f6d2f30724a773938442e676966" width="600" height="338" />

**Presenting: [https://github.com/Pitasi/dyn-wallpaper](https://github.com/Pitasi/dyn-wallpaper)**

It's not a revolution, live wallpapers existed before. But Apple did them right: a bunch of photos of the same spot in different time of the day, blended together to create the illusion of time passing - inside your desktop.

I think it's pretty cool and I wanted it, but of course I don't have a MacBook, so...write it by yourself, right?

I was too busy and excited to realize that a couple of similar scripts already existed - but they aren't complete, they are missing the "blend" the images together, just changing wallpaper every X minutes or so.

In any case, I did it! It's a really simple python script that uses [PIL](http://www.pythonware.com/products/pil/), [Astral](https://pypi.org/project/astral/), and some math to get your dawn and dusk times, generate the current time wallpaper by blending two pictures, and finally setting it as your current wallpaper. Repeat this process every 10 minutes or so, and you get my script.

GitHub repo: [https://github.com/Pitasi/dyn-wallpaper](https://github.com/Pitasi/dyn-wallpaper)

Out of the box it supports Windows (tested on Windows 10), and `feh` but you can use any command you like with the `-c` flag, check the README for some details and examples.

In the README you'll find a link to download the wallpaper set (16 images, in JPEG format), and a couple of instructions for running the script.

Also, any suggestion or pull request is welcome, of course
