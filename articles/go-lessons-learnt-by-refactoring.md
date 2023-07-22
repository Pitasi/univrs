---
title: "Go lessons learnt by refactoring"
datetime: 2020-02-21T21:35:35.000Z
---

<p>
<em>
If you didn't, take a look at:
<a href="https://github.com/golang/go/wiki/CodeReviewComments" class="break-all">
	https://github.com/golang/go/wiki/CodeReviewComments
</a>
</em>
</p>

A little context: I recently moved to a new company and they are starting using
the **Go language** for some of the projects currently developing.
I found myself reading a bunch of code written by people who only recently
started writing Go, and came from years of experience in other languages - i.e.
Java and C++.

After asking for a refactor of the code before it gets too tangled, I've seen
some non-idiomatic patterns emerge and I'm documenting them here for future
references.
We starts from basics but you may found them interesting as well!


## Names cluttering

This one is really common and already discussed everywhere but I'm facing it on
a daily basis so I'm going to repeat it here.

Always think about how a function or type is going to be referenced from the
outside:

**Don't**
```go
package services

func NewService() { /* ... */ }

// services.NewService()
```


**Do**

```go
package services

func New() { /* ... */ }

// services.New()
```

I also think that something like this is correct:

```go
package telegram

func NewClient() { /* ... */ }

// telegram.NewClient()
```

Indeed this helps understanding what it's being initialized when not already
clear from the package name.

## Slices initialization

**Don't**
```go
slice := []int{}
// or
slice := make([]int, 0)
```

**Do**
```go
var slice []int

// you can still append(), even if list is nil
slice = append(slice, 4)
```

The reason is that the first snippet will initialize a slice with length 0 and
it's really not necessary most of the time.

A `nil` slice will act just like a zero-length slice, without the useless memory
allocation.


## Change behaviour through composition

This is a method actually used in Go stdlib `sort` package, where there's a
`Interface` with some common methods, and a `reverse` struct that changes the
behavior for one of the methods of `Interface` to make it sort reversed.

In this example, I'm showing how easy it is to take a `Stringer` and return a
`Stringer`, meaning that anywhere a Stringer was used it's still valid, but it
will print the string uppercase.

```go
type upperStringer struct {
	fmt.Stringer
}

func (us *upperStringer) String() string {
	return strings.ToUpper(us.Stringer.String())
}

func Upper(s fmt.Stringer) fmt.Stringer {
	return &upperStringer{s}
}
```

This is made possible by embedding an interface so that my custom String()
method can call the original String() method.


## Check your receiver pointer for nil

**Don't**
```go
type T struct{
	name string
}

func (t *T) PrintName() {
	fmt.Println(t.name)
}
```

You should check for the pointer, and take some kind of action instead of
panicking:

**Do**
```go
type T struct{
	name string
}

func (t *T) PrintName() {
	if t == nil {
		fmt.Println("A nil has no name")
	} else {
		fmt.Println(t.name)
	}
}

func main() {
	var t *T
	t.PrintName()
	// Output: A nil has no name
}
```


## Avoid unneeded singleton pattern

(or be safe and implement it correctly)

**Don't**
```go
package foo

type Foo struct{}

func (f *Foo) Bar() {
  // do stuff
}

func GetFoo() *Foo {
	return newFoo()
}

// foo is the singleton instance.
var foo *Foo

func newFoo() *Foo {
	if foo != nil {
		return foo
	}
	foo = &Foo{}
	return foo
}
```

Let's start with a real bug I encountered refactoring this: **`GetFoo()` is not
thread-safe!** If you have a bunch of goroutines booting and asking for their
Foo, you are ending up with race conditions.

But instead of adding a mutex, let's step back. **Do you need a singleton?**

Go, just like a lot of other languages, doesn't force you to create a class for
everything. If I can, I'd prefer to just have a package foo with the functions
I need.

**Do**
```go
package foo

func Bar() {
}
```

This is so much cleaner, isn't it?

If you're wondering how can you mock the foo package for testing, I'll just
show you in the following sections - promise, but before doing that I need to
show a neat trick:


## A function can satisfy an interface by calling itself 

```go {hl_lines=["12-17"]}
package io

type Reader interface{
	Read(p []byte) (n int, err error)
}



package main

// ReaderFunc wraps a simple function into an io.Reader.
type ReaderFunc func(p []byte) (n int, err error)

func (f ReaderFunc) Read(p []byte) (n int, err error) {
	return f(p)
}



// Mock is a function that acts like Reader.Read().
func Mock(p []byte) (n int, err error) {
	s := []byte("hello\n")
	n = copy(p, s)
	return
}

func main() {
	// you cannot:
	// bufio.NewReader(ReaderMock)

	// but for example you can now:
	r := bufio.NewReader(ReaderFunc(Mock))

	l, _ := r.ReadString('\n')
	fmt.Println(l)
}
```

We actually made a function to satisfy the Reader interface by calling itself.
If you are a little confused that's ok, just read it again a couple of times
and remember that `ReaderFunc(Mock)` is a type cast, not a function call.



## Testing without singletons and other OOP patterns

So back to the singleton problem. We saw how to remove a useless type, the new
problem is that now we can't easily mock that methods:

**Before**
```go
// foo.go

package foo

type IFoo interface {
	Bar()
}



// main.go

package main
func Run(f IFoo) {
	f.Bar()
}

func main() {
	Run(foo.GetFoo())
}



// main_test.go

package main

func TestRun(t *testing.T) {
	Run(&MockFoo{})
}

type MockFoo struct {}

func (f *MockFoo) Bar {
	fmt.Println("Mocking Bar()")
}
```

But it turns out that's not an idiomatic way of thinking in Go. There is **A
LOT** to talk about interfaces and how they are different from other languages
but let's just relax and focus on our test.

We have these files:

**Now**

```go {hl_lines=["29-31"]}
// foo.go

package foo

func Bar() { /*...*/ } 




// main.go

package main

func Run() {
  foo.Bar()
}

func main() {
  Run()
}



// main_test.go

package main

func TestRun(t *testing.T) {
  /* ???????? */
}
```

The problem is that Run() strictly depends on foo.Bar() while it should not.

Before, Run used a dependency injection of an IFoo. In more idiomatic way, we
can say that *anyone that can Bar()* is sufficient - let's call it a
**Barrer**:

```go {hl_lines=["15-20"]}
// main.go

package main

type Barrer interface{
	Bar()
}

func Run(b Barrer) {
	b.Bar()
}

func main() {
	type fooBarrer struct{}
	func (f *fooBarrer) Bar() {
		foo.Bar()
	}

	Run(&fooBarrer{})
}
```

But having to define a new type inside the main seems to overcomplicate things.
Applying the previous section we can have a function implements the Barrer
interface:

```go {hl_lines=["10-14", "21"]}
// main.go

package main

type Barrer interface{
	Bar()
}

type BarrerFunc func()

func (f BarrerFunc) Bar() {
	f()
}

func Run(b Barrer) {
	b.Bar()
}

func main() {
	Run(BarrerFunc(foo.Bar))
}
```

At this point, testing Run is trivial:

```go
// main_test.go

func TestRun(...) {
	Run(BarrerFunc(func() {
		// ... do some tests
	}))
}
```

I'm trying to keep this example as simple as possible and it doesn't really
feel necessary to have a Barrer and a BarrerFunc, but you have to trust me that
as complexity grows, you can benefit a lot from this pattern.


### Talking about interfaces

**How is IFoo different from Barrer?** First of all, the name - an IFoo is
a set methods implemented by Foo. A Barrer is just someone that can Bar.

If you still don't see the difference, an interface like IFoo is implemented
only once by Foo and it's going to grow indefinitely over time. But our Run
only needed one of these methods!

There's also an important difference that could get unnoticed from my examples.
IFoo was defined in the foo package, together with its class. And that's normal
because of how tightly the interface is coupled to its implementation.

In Go **you don't do that!** The Barrer interface is defined nearly where it's
used: in the main package. That's possible thanks to Go implicit satisfaction
of interfaces, package foo doesn't need to know what interfaces it implements.

If you think about it, that's truly different from the main object-oriented
programming such as Java. And in my opinion, is one of the peculiarities that
make Go such a beautiful language.
