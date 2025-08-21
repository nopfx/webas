---
base_resource: pages/parts/post.htnl
title: Tiny result library in go
slug: go_result_lib.html
author: nop.f(x)
date: July 21 2025
size: 2.4K
intro: Why I Wrote a Tiny Result Library in Go (and Why It Helps Me Keep My Sanity)
---

# Tiny Result library in go

GIT: [GResult](https://github.com/nopfx/gresult)

Go sometimes can be annoying for me—especially when talking about error handling. So that’s why I created this little result library.

Let me be clear: I love how simple and pragmatic Go is. But I also hate how noisy and repetitive error handling can get. Writing `if err != nil { return err }` twenty times in a single function is not only boring—it also feels like I'm programming in assembly when I signed up for a higher-level language.

So I wrote a `Result[T]` type. It’s small, generic (thanks to Go 1.18+), and gives me a taste of the expressive power you get in Rust or even modern JavaScript. Let me show you how it works and explain why I think it's a good (but not perfect) abstraction.

The code:

```
package result

import "fmt"

type Result[T any] struct {
	val T
	err error
}

func From[T any](val T, err error) Result[T] {
	if err != nil {
		return Err[T](err)
	}
	return Ok(val)
}

func Ok[T any](val T) Result[T] {
	return Result[T]{val: val, err: nil}
}

func Err[T any](err error) Result[T] {
	var none T
	return Result[T]{val: none, err: err}
}

func (r Result[T]) IsErr() bool {
	return r.err != nil
}

func (r Result[T]) IsOk() bool {
	return r.err == nil
}

func (r Result[T]) Unwrap() T {
	if r.err != nil {
		panic(fmt.Sprintf("called Unwrap on Err: %v", r.err))
	}
	return r.val
}

func (r Result[T]) UnwrapOr(defaultVal T) T {
	if r.err != nil {
		return defaultVal
	}
	return r.val
}

func (r Result[T]) Map(f func(T) T) Result[T] {
	if r.err != nil {
		return r
	}
	return Ok(f(r.val))
}

func (r Result[T]) AndThen(f func(T) Result[T]) Result[T] {
	if r.err != nil {
		return r
	}
	return f(r.val)
}

```

## What’s Good About This?

### Cleaner Chaining

Instead of doing this:

```
res, err := doThing()
if err != nil {
    return err
}
res2, err := doAnotherThing(res)
if err != nil {
    return err
}
```

You can do this:

```
result.From(doThing()).
    AndThen(doAnotherThing).
    Unwrap()
```

Much more readable and declarative. It lets me focus on the data flow, not on boilerplate.

### Safety with Panic Option

The `Unwrap()` method panics if the result contains an error, which is perfect for quick scripts or test scenarios where you don’t want to do verbose error checks. If you want a fallback, there's `UnwrapOr()`.

This duality is inspired by Rust's Result, and it feels right.

### Generic and Reusable

By leveraging Go’s generics, this works with any type. You get all the benefits of type safety, without the need for type assertions or reflection tricks.

### Functional Flavor

`Map()` and `AndThen()` give you a taste of functional composition. `Map()` transforms the inner value. `AndThen()` lets you chain more Result-producing functions. This opens the door to writing expressive pipelines without tangled conditionals.

I built this Result library not to rewrite Go’s philosophy, but to cope with it. It’s a small abstraction that helps me write cleaner, more composable code—especially in projects where performance isn’t critical and developer happiness matters more.

If you’re like me and error handling in Go starts to feel like busywork, give it a shot. Just remember: abstractions are tools, not rules. Use them where they help. Ditch them where they hurt.

Let me know what you think—or better yet, send a PR. 😄

GIT: [GResult](https://github.com/nopfx/gresult)
