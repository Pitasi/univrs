---
title: "Learning programming with a turtle"
datetime: 2023-08-24T08:40:00.000Z
---

I had the incredible fortune (I appreciate it now) of having a computer science class in elementary school when I was a child. Not only that, but the teacher was a true computer scientist, not a math teacher who improvised as a computer scientist.

I've heard of folks starting their computer programming vocations when they were quite young. Amiga, Commodore 64, and more brands I've have not encountered of. However, floppy disks were still in use in the early 2000s. I realise I sound like those elderly people who say "back in my days..." but I recall buying those packs of virgin floppy disks and personally handing them over to my teacher for submitting our assignments for him to evaluate.

We are all aware that "learning by doing" has some value. My teacher was also aware of this. In fact, we used [MSWLogo](https://en.wikipedia.org/wiki/MSWLogo) in our courses (currently renamed and maintained as [FMSLogo](https://fmslogo.sourceforge.io/)).

Keep in mind that we're talking about the early 2000s in Italy (first world, but not the most technologically advanced country) in an elementary school with children aged 6 to 11. I'm sure he was one of the few teachers in the country doing so (I might be incorrect; I cannot find any statistics). I recognize now how fortunate I was and how much of an influence this made on me.

If you've never heard of [Logo](https://en.wikipedia.org/wiki/Logo_(programming_language)), it's an interpreted programming language that we employed for [turtle graphics](https://en.wikipedia.org/wiki/Turtle_graphics), designed expressly for educational reasons. Because learning by doing is more enjoyable and interesting.

So, where is the turtle? The turtle, is a little triangle on a white canvas. The turtle also carries an invisible pen (fortunately, children have imagination).

<image avif="https://assets.anto.pt/articles/learning-programming-with-a-turtle/fmslogo.avif"
    webp="https://assets.anto.pt/articles/learning-programming-with-a-turtle/fmslogo.webp"
    png="https://assets.anto.pt/articles/learning-programming-with-a-turtle/fmslogo.png"></image>

You can use the REPL to run instructions that cause the turtle to move (forward or backward), rotate (by how many degrees), and raise or lower the pen.

If the turtle moves without lifting the pen, it will draw a line on the canvas.

You can start to experiment with it online at https://inexorabletash.github.io/jslogo/. Despite the fact that I'm little disappointed to see a turtle icon instead of a triangle.

Let's go over some instructions; I've not included screenshots since, in the spirit of this post, I expect you to open the preceding link and execute it yourself.

Move the turtle forward:
```
fd 10
```

Clear the screen:
```
cs
```

Draw a square:
```
cs
fd 100
rt 90
fd 100
rt 90
fd 100
rt 90
fd 100
rt 90
```

Easy, right?

We can also do it with loops, in a single line:
```
cs
repeat 4 [fd 100 rt 90]
```

Now, let's draw the simplest triangle:
```
cs
repeat 3 [fd 100 rt 120]
```

Finally, to finish on a high note with the item that blew my mind as a youngster, let's draw a lot of triangles:
```
cs
repeat 1000 [
  repeat 3 [fd 100 rt 120]
  rt 15
]
```
(Note that you might have entered 24 instead of 1000 because 360/15=24, are we learning...math?).

If you want to have even more fun, replace 15 with 1.

That concludes this brief article. I wanted to write about how I initially became interested in programming, and how "learning by doing" can be a lot of fun, which is frequently missed in contexts such as math classrooms at schools and universities.

My takeaway: don't just learn about Rust's borrow checker and think you've got it. Go make something with it. (*Of course, this is true for everything, not just Rust)*
