---
title: "How to use a cheap USB token (or Yubikey) instead of password in Arch Linux"
datetime: 2018-03-23T14:20:31.000Z
---

I bought a cheap yet powerful USB U2F token from Amazon.it ([link](https://www.amazon.it/Feitian-ePass-FIDO-NFC-Security-Key/dp/B01M1R5LRD)). The U2F standard is compatible with a few websites, they are listed here: .

**Beware: I'm using *Plasma* and *SDDM*, I did not test all of this with other DEs or DMs.**

Also, pay attention before doing something possibly irreversible!

![](https://images.unsplash.com/photo-1520861701132-bbcd1385d171?ixlib=rb-0.3.5&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=1080&fit=max&ixid=eyJhcHBfaWQiOjExNzczfQ&s=1195d56d35102c2f3f39917ba2077359)  
Photo by [Nick Fewings](https://unsplash.com/@jannerboy62?utm_source=ghost&utm_medium=referral&utm_campaign=api-credit) / [Unsplash](https://unsplash.com/?utm_source=ghost&utm_medium=referral&utm_campaign=api-credit)

Mostly thanks to , what I also was able to get is to use this token instead of my password (but if you do care your privacy, you should use this as a second factor - not the only one, to do that you only need to change a word in the PAM module).

Since we are going to use PAM, you can also use your token instead of your password for `sudo` commands and unlocking KDE/Plasma lockscreen.

Plus, using a simple Udev rule, I was able to automatically lock the screen when the USB token is plugged off from my computer (I can still remove it without locking my screen holding Shift).

Setup
-----

Thanks to the magic of AUR and the effort of Yubico, the process it's absolutely simple. First of all, remove the usb key from your computer and install the PAM module.

```
$ pacaur -S pam_u2f

```

Now plug in the token and run:

```
$ pamu2fcfg -uantonio
antonio:longstuff...

```

where `antonio` is my Linux username.

Create and put the result of this command in `/etc/u2f_mappings`, stripping the final `%` character if you have it.

---

PAM
---

We're almost there.

Edit `/etc/pam.d/system-auth` file and add, before any other *auth* line (note that this is a single line):

```
auth  sufficient  pam_u2f.so  debug authfile=/etc/u2f_mappings cue

```

Basically **sufficient** means that the pam\_u2f module is enough for logging in the user and the password won't be asked. You can change it to **required**, and put that line right after the *pam\_unix* one, in order to request both the password and the second factor. **Try your USB token in *sufficient* mode before changing it to *required*.**

**debug** is an useful way of getting more informations in case something go wrong, you can remove this later.

More informations about this PAM module: .

Reboot your computer and you are done!

Now from the login/lock screen you should be able to enter an empty password and tapping your token for loggin in :)

![](https://images.unsplash.com/photo-1484814915025-858e00100866?ixlib=rb-0.3.5&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=1080&fit=max&ixid=eyJhcHBfaWQiOjExNzczfQ&s=453aaa9a8c3d972be1e04348401b83d6)  
Photo by [Cristina Gottardi](https://unsplash.com/@cristina_gottardi?utm_source=ghost&utm_medium=referral&utm_campaign=api-credit) / [Unsplash](https://unsplash.com/?utm_source=ghost&utm_medium=referral&utm_campaign=api-credit)

Locking the screen when the token is removed
--------------------------------------------

Now this part was a little bit trickier, but I got there at the end.

We are gonna use Udev, and `evtest` for checking if the Shift key is being pressed. So be sure to run:

```
$ sudo pacman -S evtest

```

Create the file `/usr/local/bin/yubikey-lock-screen` with this content:

```
#!/bin/bash

HOTKEY="KEY_LEFTSHIFT"

# Write message to system log
/usr/bin/logger "Screen locked because Yubikey has been disconnected."

# Check, if hotkey is not beeing pressed during lockscreen attempt
kbd_devices=`cat /proc/bus/input/devices | egrep '^H:.* kbd ' | sed 's/.*event\\([0$
for event_dev in ${kbd_devices}; do
    evtest --query /dev/input/${event_dev} EV_KEY ${HOTKEY}
    if [[ "$?" != "0" ]]; then
        exit 0
    fi
done

# Lock the screen
/usr/bin/loginctl lock-sessions

```

I made this editing a bit the script you can find [here](https://github.com/nshadov/yubikey-kde-screensaver/blob/master/yubikey-lock-screen). Thank *nshadov*!

Now give it permission to be executed:

```
$ sudo chmod a+x /usr/local/bin/yubikey-lock-screen

```

and now you can just run `/usr/local/bin/yubikey-lock-screen` to check if it really locks your screen.

Now we're going to add a trigger that will call this script, and here comes the tricky. Run:

```
$ udevadm monitor --property

```

remove your USB token, and check the **UDEV** (not Kernel) events saying action **remove**. My device didn't have a line containing the *vendorId* and *productId*, but I found this:

```
PRODUCT=96e/858/4004

```

So what you're looking for is a line containing a variable that can identify your token.

At the end, my udev rule is a single line:

```
SUBSYSTEM=="usb", ACTION=="remove", ENV{PRODUCT}=="96e/858/4004", RUN+="/usr/local/bin/yubikey-lock-screen"

```

Put this in `/etc/udev/rules.d/71-autolockscreen.rules` and load the new rule with:

```
$ udevadm control --reload
$ udevadm trigger

```

Done.

We did it
---------

I hope this will help someone, but also I hope that a future me will find this notes useful next time I'll format my hard drive.

See you soon!
