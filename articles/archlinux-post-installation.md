---
title: "Archlinux Post Installation"
datetime: 2018-08-14T14:39:06.000Z
---

If you are not using Arch, you probably should. There's a lot to be said about Arch Linux but that's not the right place: we are here to install what I think it's basic stuff to get your brand new installation up and running.

**Yep**. There's already the **official wiki (aka The Bible)** guide here: [https://wiki.archlinux.org/index.php/installation_guide](https://wiki.archlinux.org/index.php/installation_guide). It's the place you have to look at, if you're installing Arch for your first time.

What I'm going to write here is a note to myself for every post-installation package and every setup I need in order to have a **beautiful** and **clean** system. I could probably just write a script for doing that every time and maybe I'll do it in the future, but since this is an always **work in progress** I'm happy with this for now.

## AUR helper

I'm currently using Yay, manually install it from AUR:

```
    cd /tmp
    git clone https://aur.archlinux.org/yay.git
    cd yay
    makepkg -si
```

### Enable colors

This will enable color output for both `pacman` and `yay`.

```
    vim /etc/pacman.conf
    # Uncomment the "Color" line
```

### Faster package creation

By default `makepkg` create the package, the compress it. It is cool if you want to share it to others but probably you (or `yay`) are just going to install it for yourself - it's an useless and slow step. We can avoid it like so:

```
    vim /etc/makepkg.conf
    # Edit the penultimate line to be like:
    # PKGEXT='.pkg.tar'
```

## Blazing fast internet startup

I used NetworkManager for a long time, then I found someone on Reddit suggesting `connmanctl` because it's so much faster at startup to connect - and it's true. So I suggest to install it, together with a simple GUI:

```
    yay -S connman connman-gtk

    # Be sure to stop and disable NetworkManager or others before!
    sudo systemctl enable connman
    sudo systemctl start connman
```

You can also start `connman-gtk` at startup if you want the tray icon.

## DE, fonts, etc.

I'm currently using i3wm but I also keep Gnome installed because I like breaking things and I need a backup.

```
    yay -S gnome gnome-tweaks  # for gnome
    yay -S i3wm-gaps  # for i3wm, with gaps patch
    systemctl enable gdm
```

My personal dotfile can be found at [https://github.com/pitasi/dotfile](https://github.com/pitasi/dotfile). They are a constant work in progress so you probably would have to figure out some stuff by yourself - **DO NOT JUST COPY PASTE THEM, SOMETHING TERRIBLE WOULD HAPPEN!**

### Fonts

Some basic fonts you can install

```
    yay -S noto-fonts-emoji
```

I also use Terminus as my terminal font because bitmap fonts are sharps and great:

```
    yay -S terminus-font
```

You **must** one of these pixel sizes: [8, 10, 11, 12, 14, 16].

### Theme

Arc Dark + Papirus icons = ❤️

```
    sudo pacman -S arc-gtk-theme papirus-icon-theme
```

Use `gnome-tweaks` (or maybe `lxappearance` in i3wm) to select the theme.

Alternative (macOS inspired) icons:

```
    yay -S la-capitaine-icon-theme
```

### macOS Mojave Dynamic Wallpaper

Check [https://github.com/Pitasi/dyn-wallpaper](https://github.com/Pitasi/dyn-wallpaper), and its blog post [here!](/articles/mojave-wallpapers).

## Other packages

### Shell

I like Fish shell, so...

```
    sudo pacman -S fish
    fish
    curl -L https://get.oh-my.fish | fish
    omf install bobthefish
    chsh
    # select /usr/bin/fish
```

### YubiKey support

[See my other post](/articles/yubikey)
