---
title: "AcePC T11 ghost battery \"problem\""
datetime: 2019-11-10T15:32:32.000Z
---

## An intro

(feel free to jump to the next section)

I always wanted to have a little domestic server for simple stuff that can be
useful. In particular I wanted something similar and a bit more powerful than a
Raspberry PI, that's why my research brought me to this Chinese manufacturer
called AcePC (https://www.iacepc.com/) - I know their website it's not
promising but their computers are actually good and low cost.

The main feature that made me by a T11, was the CPU architecture, in fact it
has a fanless Intel Cherry Trail. I like that because it's typically easier to
find packages for that architecture and I can run Docker like I was on my
laptop without any problem.


## The "problem"

After the unboxing, the first thing to be done was to format the drive and
install Arch Linux. So that's what I did.

I also installed Gnome thinking that a DE may be useful in case I decide to use
this PC as a TV Box.

And that's where I first saw something weird: Gnome was telling that the
battery is discharging.

*__Wait what?__ This thing has a battery? And why is it discharging?*

I checked the specs and searched the internet to find out that no, this
computer has no batteries, and I'm not the only one with this problem - it
basically happens on every Linux installation probably because it's detected as
a tablet.

Well, that's an annoying icon but I said to myself it's okay, I do not care if
Linux thinks a battery is powering my machine. I turned off any powersave
function anyway.

_[ed.: I still don't know **why** Linux detects this battery so if you have any
information you can contact me so I can update this post]_


## The TRUE problem

Fast-forward a month of daily usage.

I'm using this AcePC as a DNS server with [Pi-Hole](https://pi-hole.net/), it's
a cool project that you should check out if never heard of it.

It happens, quite often actually, that the AcePC T11 suddenly stops working. I
can't SSH into it, and I can't surf the web because I no longer have a DNS
resolver. That's a shame, I really liked that little PC.

Everytime, I go check it and I find it completely powered off with also its LED
turned off. I have to unplug and plug it again so it power on again.

After that happened a bunch of time I was pretty annoyed so I finally took some
time to have a serious look into this.

It didn't take me that long to realise it was just powering off, gracefully (I
just ran `journalctl` and looked for `Rebooting`).
But who was initiating the shutdown routine?! *[drumroll]* ...The low battery.


## Solution

Okay so now I know where the problem is and the first thing I did was to search
into the web if somebody else was having this problem. The official AcePC forum
is basically useless because its being targeted from a massive spam attack and
nobody seems to care about it.

Lucky for me I found [this Amazon.com
review](https://www.amazon.com/gp/customer-reviews/R2E9C94U8YDGEF) that really
saved me a lot of time.

Basically, update your `/etc/UPower/UPower.conf`, setting:
```
NoPollBatteries=true
```

The `NoPollBatteries` option will make your UPower daemon rely on updates sent
by the battery itself, instead of asking the battery for its percentage
continuously. Good thing is, there is none of this events and your battery
percentage won't be updated at all.


## Moreover

I contacted AcePC but they were not helpful at all. So I think this fix is
going to stay :)
