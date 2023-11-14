---
title: Messily Thinking Pythonically
date: '2018-02-02T12:12:12-08:00'
description: I attempt to narrate my thought process solving a data transformation problem.
---

Solving a data transformation problem Pythonically, explained in far too much detail.

![python powered logo](.perseus/static/python-powered.png)

_(To see what tests for all this look like, check out my later post, [Muddled Property-based Tests](/post/muddled-property-based-tests/). This post originally appeared on Medium, but I've moved it to my personal site.)_

I’m a member of the Puget Sound Programming Python (**PuPPy**) user group, and we have a large and active Slack ([join us!](http://slack.pspython.com)). One of my favorite channels is **#q-and-a**, where anyone can ask just about anything and get help. Someone recently asked about transforming one data structure to another, and especially about how to do it Pythonically, and I was able to help out.

I’m writing this post using that example to explain some of the things I think about when **1) manipulating data** and **2) trying to write Pythonic code**. I like the example here because, unlike some situations, it is not immediately obvious what a Pythonic solution will look like.

I know that sometimes **writing good code can look to junior developers like pulling magic solutions out of a hat**, and I wanted to show how my process is very **incremental**, and involves plenty of **muddling** around when things don’t quite work like I expect. I’ve tried to recreate my actual thought process solving the problem, and provide a few rules of thumb that may assist in those facing similar situations.

Without further ado, the problem.

Given a data structure like…

```python
boop = {
  “hello”: [1, 2, 3],
  “bye”: [4, 5, 6]
}
```

How do we transform that to a data structure like…

```python
beep = [
  {“hello”: 1, “bye”: 4},
  {“hello”: 1, “bye”: 5},
  {“hello”: 1, “bye”: 6},
  {“hello”: 2, “bye”: 4},
  {“hello”: 2, “bye”: 5},
  {“hello”: 2, “bye”: 6},
  {“hello”: 3, “bye”: 4},
  {“hello”: 3, “bye”: 5},
  {“hello”: 3, “bye”: 6}
]
```

?

When I’ve got a “convert between formats” problem like this, I often** start by editing a one-liner until I reach the limits of a one-liner**, or it works, then figure out what the right abstractions are. **This is not the one right way, just one way I sometimes use**. That’s partly because I find one-liners fun, and partly because I think the process affords a good sort of “accumulative” approach, which is probably best illustrated by example.

Okay, so first, all the keys and values in boop are also present in beep. So we need to get those out of there no matter what. There’s a standard way to do that, `.items()`. What’s more, I know we’re going to want to do more with them immediately, and that to me means a comprehension, which gives me a first piece:

```python
(… for (k, vs) in boop.items())
```

(I love** explanatory variable names**, and inside a dictionary-focused comprehension, while I’m toying with ideas, k for “key” and vs for “values” are fine. They’ll be changed as things solidify.)

Okay, so we’re getting things out and doing something to them, but what are we doing? Here I start to think about what we need on the other end, plus what’s going to do the “work” of combining everything right.

Turns out the “work” is easy, the dictionaries have a product (read as: every combination) of the values in each of “hello” and “bye”; unsurprisingly, my solution will use `itertools.product()`, which makes that sort of combination. So, if we’re using `itertools.product()`, what comes out of it? That will help clarify what goes in.

I know I need to make dictionaries, and there are two main ways I like to do that Pythonically: the dict function, which can work with several forms of arguments, or dictionary comprehensions. I know what I’m working with comes out of `itertools.product()`, and the code will read simplest if there’s just a single argument to dict, so I’m going to write a partial line to see what that looks like:

```python
[
  dict(something) for something in
  itertools.product(
    ….something in here using the stuff I wrote above…
  )
]
```

I know that one convenient argument the dict function can take is an iterable of key-value 2-tuples (pairs, like `(“hello”, 1)`) and that seems like a good fit here — I’m taking keys and values and reshaping them (somehow), and there’ll probably be a convenient place to make them into 2-tuples at some point, which can then become the dictionary I need using that argument. Most of the other options for dict would need either multiple arguments (doesn’t fit the mental model I’m currently working with) or an existing dict (which would mean I’d already be done, so not what I want). Also, I often find the tuple-taking version of dict useful, just as a general observation.

Okay, that helps. Since I know I want something to be an iterable of key-value 2-tuples, that means each iterable product takes needs to be of key-value 2-tuples, because all it’s going to do is reshuffle what I provide. I’ve currently got keys and lists of values… time for more comprehensions! (I might like comprehensions a little much.)

Okay, so with k and vs I can write… ((k, v) for v in vs), and then I can drop that into the first partial piece I wrote:

```python
(((k, v) for v in vs) for (k, vs) in boop.items())
```

Notice the opening parentheses (**generator comprehensions**!) are already getting ridiculous. Don’t worry, when it comes time to break stuff up that will be dealt with. But before then… time to make things more ridiculous!

Stepping back, we have an iterable, where each value is an iterable of key-value 2-tuples. That sounds an awful lot like what we need to pass to product! Let me see, example values in the inner iterables will be things like (“hello”, 2) and (“bye”, 4) (if this isn’t clear, let me know and I’ll edit to elaborate on why), so that seems to already be the sort of thing we’re expecting later. Now how do we get product to do all the things?

What if we just take the above and stick it into product?

```python
>>> itertools.product(((k, v) for v in vs) for (k, vs) in boop.items())

<itertools.product object at 0x103034360>
```

Oops, forgot to expand things for looking at in the REPL…

```python
>>> list(itertools.product(((k, v) for v in vs) for (k, vs) in boop.items()))

[(<generator object <genexpr>.<genexpr> at 0x10302b8e0>,), (<generator object <genexpr>.<genexpr> at 0x10302b938>,)]
```

Okay, still can’t see the values, but wait, there are only two members of the list. We need one value per dict we expect in the output, and that is more than two…

Time to go look at the itertools.product() documentation and see why it isn’t behaving like needed. Oh! Each iterable needs to be a separate argument, but we’re providing a single iterable that has all the other iterables inside it. Luckily Python has a way to project an iterable into the arguments of a function: prefix it with a `*` (look closely below, it’s a very small change).

So…

```python
>>> list(itertools.product(*(((k, v) for v in vs) for (k, vs) in boop.items())))

[((‘bye’, 1), (‘bye’, 4)), ((‘bye’, 1), (‘bye’, 5)), ((‘bye’, 1), (‘bye’, 6)), ((‘bye’, 2), (‘bye’, 4)), ((‘bye’, 2), (‘bye’, 5)), ((‘bye’, 2), (‘bye’, 6)), ((‘bye’, 3), (‘bye’, 4)), ((‘bye’, 3), (‘bye’, 5)), ((‘bye’, 3), (‘bye’, 6))]
```

Oh, that looks perfec… wait. What’s with all the keys being “bye”??? Now, this is something I’ve run into before, so I know it has to do with Python’s scoping rules. For now we’re going to fix it by turning one of the generator comprehensions into a list comprehension, making it concrete and immediate (instead of lazy). Look for where `[]` are used instead of `()`:

```python
>>> list(itertools.product(*([(k, v) for v in vs] for (k, vs) in boop.items())))

[((‘hello’, 1), (‘bye’, 4)), ((‘hello’, 1), (‘bye’, 5)), ((‘hello’, 1), (‘bye’, 6)), ((‘hello’, 2), (‘bye’, 4)), ((‘hello’, 2), (‘bye’, 5)), ((‘hello’, 2), (‘bye’, 6)), ((‘hello’, 3), (‘bye’, 4)), ((‘hello’, 3), (‘bye’, 5)), ((‘hello’, 3), (‘bye’, 6))]
```

Now it works right. I was pretty sure that would work because what was happening was, k was only showing the most recent value (“bye”), and it was being passed into an inner comprehension, so I made it so each time we got a different k, we made it concrete with a list comprehension. If you see something like this when playing with a one-liner, instead of trying to figure out exactly how the scoping rules work, you can just **turn every comprehension into a concrete one** (list, dict, set) instead of a generator comprehension. You can always back out of that later when you break things up, which often solves the scoping problem separately by introducing new functions.

Okay, so what comes out of this is an bunch of iterables of 2-tuples, such as `((‘hello’, 1), (‘bye’, 4)) and ((‘hello’, 2), (‘bye’, 4))`. That’s exactly what we need for dict, which means we can use our earlier formulation!

```python
[dict(something) for something in itertools.product(*([(k, v) for v in vs] for (k, vs) in boop.items()))]
```

And the output is exactly beep,

```python
[{‘hello’: 1, ‘bye’: 4}, {‘hello’: 1, ‘bye’: 5}, {‘hello’: 1, ‘bye’: 6}, {‘hello’: 2, ‘bye’: 4}, {‘hello’: 2, ‘bye’: 5}, {‘hello’: 2, ‘bye’: 6}, {‘hello’: 3, ‘bye’: 4}, {‘hello’: 3, ‘bye’: 5}, {‘hello’: 3, ‘bye’: 6}]
```

Now that we have something working, how do we make it not an unreadable mess?

First, time to return to names. We’re going to be making this a function (**functions are very Pythonic**), and that function needs a name. Tempting as boop_to_beep is, resist. As I was working through this, I started thinking of “hello” and “bye” as kinda like categories we needed to find every combination of. So, a decent function signature (absent other information) might be:

```python
def combos(categories):
  return ...(see above)...
```

Next, time to break it up. One rule I have is, **I do not like reading nested comprehensions**, so I’ll start there. I do like comprehensions that look like: `[func(thing) for thing in things]`, so if I can come up with an understandable function to make the inner comprehension, I can use that construct. What does that function do? It turns a key and multiple values into a bunch of pairs. Not the worst idea for a function name:

```python
def pairs(key, values):
  return ((key, value) for value in values)
```

Not too bad, but this is not a totally general function for making pairs. It works for a pretty specific situation. At least, **when I look at the function signature I don’t immediately know** how pairs would be made for a single key and some values. (I ask myself this sort of question a lot when writing code. ) Rather, it makes the pairs we need for categories. So, maybe this way of writing it?

```python
def pairs(category):
  key, values = category
  return ((key, value) for value in values)
```

Now the signature tells me this is pairs for a category, and I can see right below that a category is a key and some values, and how we turn those into pairs. That’s also convenient for where we’re calling it, isolating the unpacking inside the function:

```python
def combos(categories):
  return [dict(something) for something in itertools.product(*(pairs(category) for category in categories.items()))]
```

That line is way too long and complex to read, what should we pull out? Another kinda-rule I have is, **I like using \* to expand arguments that are variables** instead of larger expressions. That and a little thought on naming leads to:

```python
def combos(categories):
  category_pairs = (pairs(category) for category in categories.items())
  return [dict(something) for something in itertools.product(*category_pairs)]
```

Hmm, I could stop there, but itertools.product(), **the workhorse function, which is key to understanding how this function works, is a bit buried**, plus that last line is still a bit complicated for readability. Maybe…

```python
def combos(categories):
  category_pairs = (pairs(category) for category in categories.items())
  pair_combos = itertools.product(*category_pairs)
  return [dict(combo) for combo in pair_combos]
```

And there we go, that’s the solution I sent. In full:

```python
import itertools


def pairs(category):
  key, values = category
  return ((key, value) for value in values)


def combos(categories):
  category_pairs = (pairs(category) for category in categories.items())
  pair_combos = itertools.product(*category_pairs)
  return [dict(combo) for combo in pair_combos]
```

Initially when I wrote this post, I just ended with that, but I thought I might try to sum up a bit. **Writing code is messy for everyone, even if some of us have more experience navigating the mess**. One of my strengths is, I can juggle a lot of mental models in my head — such as what the data will look like at different points. You can see that above, in how I write about assembling all these pieces. This might be your strength too, or you might have different ones that let you slice through this problem in some other way. However you solve a problem, working incrementally, trying pieces out once you get something runnable, and consulting documentation whenever a library doesn’t act like you expect, like I do above, are all great tools in your belt, whether you have fifteen years experience or five months.
