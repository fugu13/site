---
title: Property-based Testing from Scratch (in Python)
date: '2018-02-13T12:12:12-08:00'
description: Writing a property-based test with no framework to understand how they work, and how they're useful for general application programming.
---

Using a single, short property-based test written without a framework to guide the development of a tricky piece of date logic involving leap years.

![a record being scratched](.perseus/static/record.png)

_(This post is based on the first chapter of a book I’m working on to introduce property-based testing for practical application. It originally appeared on Medium, but I've moved it over to my personal site.)_

**Your own worst enemy: your past self**

I’ve lost count of how many times a programmer has told me about their own past code with a pained look on their face. I’ve definitely written a _lot_ of code that is full of flaws, from small bugs to complete failures of logic. Sometimes old code I wrote feels like I’m deliberately avoiding the right way to do things!

Imagine your past self wrote a first take on an age-checking function.

```python
import datetime


def check_age(birthday, today):
  return birthday + datetime.timedelta(days=365 * 21) <= today
```
This does not look that bad. I mean, it isn’t right, and if you’ve done much with calendars you can probably point at a likely problem area (leap years), but this seems like a great starting point. Time to write some tests! Maybe a few simple ones to get started…

```python
import datetime

from agecheck import check_age


def test_allows_old_enough():
  birthday = datetime.date(1990, 5, 15)
  today = datetime.date(2017, 1, 1)
  assert check_age(birthday, today)


def test_forbids_too_young():
  birthday = datetime.date(1990, 5, 15)
  today = datetime.date(2000, 1, 1)
  assert not check_age(birthday, today)


if __name__ == '__main__':
  test_allows_old_enough()
  test_forbids_too_young()
```

I’m not using any testing framework at all, but of course you should. What I’m trying to do here is introduce the idea of property-based testing completely from scratch, so all this code runs without installing a single library. I’m not using the built-in unittest framework both to avoid boilerplate and because it doesn’t look much like future code will, with the libraries pytest and hypothesis. Just put all the files in the same directory and run the test file to run the tests.

But anyways, that wasn’t so bad! Okay, now what are all the edge cases that find those leap year problems? Let me see, if someone was born really near the beginning of a month, then our math won’t work because by their 21st birthday there have been multiple leap years, which means… wait, why are we writing tests with knowledge of how our function is implemented? Even if that ends up working, isn’t that an odd way to go about things? What if I wasn’t already used to thinking about leap years?

**Save your brainpower**

Good tests are about saving brainpower. Instead of having to already basically understand the problem we’re looking for, we want tests to uncover new problems for us. A property-based test can do that. Without focusing too much on the details of the code or what problems the code might or might not have, what are the entities and relationships our tests are manipulating? What properties do those have?

Okay, one kind of entity basically jumps out in this code: birthdays. What are some properties birthdays have? Well, they’re always in the same month as the month a person was born in, right? The code doesn’t make it easy to work with that 21st birthday, though. It hides inside check_age. What if first we extract out a function that calculates 21st birthdays, and focus our testing on that?

```python

import datetime


def check_age(birthday, today):
  return twenty_first(birthday) <= today


def twenty_first(birthday):
  return birthday + datetime.timedelta(days=365 * 21)
```

Now how about we write another test…

```python
import datetime

from agecheck import twenty_first


def test_twenty_first_same_month():
  birthday = datetime.date(1990, 3, 6)
  assert birthday.month == twenty_first(birthday).month


if __name__ == '__main__':
  test_twenty_first_same_month()
```
Not bad, not bad… but how many of those do we need to write to really check this property? Also, are we writing them for the right dates? How is this better than just writing more of the sorts of tests we started with?

**Your own better opponent**

A better opponent is different from a worst enemy. Your enemy undermines you at every turn, throwing up roadblocks, but while a better opponent still works against you, they challenge you to be better instead of finding ways to mire you in muck. When you write a property-based test, you’re creating a better opponent for yourself. The properties in the test tell the opponent how to prove your code wrong, guiding you to the complete implementation.

Tests are a good opponent even when not property-based, but the difficulty with most tests is, they aren’t able to challenge you: you had to know what was going to happen in order to write the test, like above with specific dates and leap years. Property-based tests, however, guide the computer to come up with new ways to challenge you. That’s what makes them an opponent that’s better than you.

That’s why even though we’re checking a property, this new test is no more satisfying than before. We’re missing something crucial. We have to generate new challenges for our code. Eventually we’ll use a library to help, but to understand the basics, we’ll continue implementing everything.

Don’t worry about everything in it for now, we’ll explain it in detail soon, just take a quick look.

```python
import datetime
import time
import random

from agecheck import twenty_first


# Our property-based test! It generates a lot of examples, and then it checks
# the property we've identified, being in the same month, for each of them.

def test_must_be_in_same_month():
  # Okay to check lots, because our assert will stop us at the first problem
  for ii in range(10000):
    birthday = random_day()
    error_message = """{} doesn't have the right twentyfirst birthday!
      Instead it has {}""".format(birthday, twenty_first(birthday))
    assert birthday.month == twenty_first(birthday).month, error_message


# Generators! These help us make what we'll use in our tests.
# Since our function takes birthdays, we need to make
# a generator for them.

def random_day():
  date_min = datetime.date(1970, 1, 1)
  date_max = datetime.date(2037, 12, 31)
  random_posix_time = random.randrange(to_posix(date_min), to_posix(date_max))
  return from_posix(random_posix_time)


def from_posix(seconds):
  return datetime.date.fromtimestamp(seconds)


def to_posix(date):
  return time.mktime(date.timetuple())


if __name__ == '__main__':
  test_must_be_in_same_month()
```
Okay, go ahead and run it, and right away you will probably see something like

  > AssertionError: 2013–11–04 23:16:23 doesn’t have the right twentyfirst birthday!
  > Instead it has 2034–10–30 23:16:23

Oh! That’s exactly the leap year thing we were worried about.

Okay, now that we have a failing test (that we didn’t need to invent by knowing about leap years in advance), how about we figure out a fix? Maybe something simple will work. Leap years are usually every four years, so between birth and 21st birthday there should be five leap years, meaning five extra days, right? Okay, so how about a small change to our twenty_first function, adding an extra five days.

```python
import datetime


def check_age(birthday, today):
  return twenty_first(birthday) <= today


def twenty_first(birthday):
  return birthday + datetime.timedelta(days=365 * 21 + 5)
```
Rerun the test with the modification, and… oh. Hmm. You probably see something like

  > AssertionError: 2035–10–01 01:31:12 doesn’t have the right twentyfirst birthday!
  > Instead it has 2056–09–30 01:31:12

Run it a few times more and you’ll probably notice a common pattern. The most frequent problem you’ll see is when a birthday is on the 1st of a month, but for some reason the 21st birthday is being put on the end of the previous month. What’s going on? We can’t just add a sixth day (try it, you’ll see lots of failures for a birthday at the end of the month leading to a 21st birthday on the 1st of the next month).

An aside, here. This particular problem isn’t one I was already expecting when I wrote this code! I knew I’d find a number of problems, but I deliberately didn’t puzzle them all out beforehand. The property-based test found it for me.

Okay, time to figure out what the problem is. If you do the math one year at a time with the example failure above, it starts to make sense. 365 days after October 1st 2035…

- since 2036 is a leap year, that’s November 30th 2036.

- Again, November 30th 2037, 2038, 2039…

- and 2040 is a leap year again, so November 29th 2040, 2041, 2042, 2043,

- November 28th 2044, 2045, 2046, 2047,

- November 27th 2048, 2049, 2050, 2051,

- November 26th 2052, 2053, 2054, 2055,

- and then November 25th 2056, add five, and…

Huh, November 30th 2056. Same as the test found, because we calculated it how our code does, but still a surprise. Why? Look at the math. For that birthday, there aren’t five leap years, there are six: 2036, 2040, 2044, 2048, 2052, and 2056.

The whole leap year thing is a bit complicated. Not unexpected, if you’ve done calendar stuff before. Sometimes there are five leap days over a 21 year span, sometimes six (and sometimes four!). How do we know which? Luckily, the python calendar library has something for us. We’ll step a few steps ahead, then fix one last thing. First, update the code to look like

```python

import datetime
import calendar


def check_age(birthday, today):
  return twenty_first(birthday) <= today


def twenty_first(birthday):
  leapdays = calendar.leapdays(birthday.year, birthday.year + 22)  # [first, second)
  if calendar.isleap(birthday.year) and birthday.month > 2:
    leapdays -= 1
  elif calendar.isleap(birthday.year + 1) and birthday.month < 3:
    leapdays -= 1
  return birthday + datetime.timedelta(days=365 * 21 + leapdays)
```
That logic probably makes a lot of sense if you think about it. What’s more, if you just keep making sensible modifications to make specific failures work, you’ll arrive at it naturally. Now run the test again… huh, still getting an error. Run it a few times and you’ll see that the error keeps looking very similar, something like

  > AssertionError: 2024–02–29 00:19:05 doesn’t have the right twentyfirst birthday!
  > Instead it has 2045–03–01 00:19:05

Wait, when does the 21st birthday for someone born _on_ a leap day occur? In a real life situation, you’d figure out what legal standard matters for your company. Now, since it turns out in practice this is mostly not handled by the law, we’re just going to make an arbitrary decision: in non-leap years, the birthday for someone born February 29th is February 28th. Add that logic and our code looks like

```python

import datetime
import calendar


def check_age(birthday, today):
  return twenty_first(birthday) <= today


def twenty_first(birthday):
  leapdays = calendar.leapdays(birthday.year, birthday.year + 22)  # [first, second)
  if calendar.isleap(birthday.year):
    if birthday.month > 2 or (birthday.month == 2 and birthday.day == 29):
      leapdays -= 1
  elif calendar.isleap(birthday.year + 1) and birthday.month < 3:
    leapdays -= 1
  return birthday + datetime.timedelta(days=365 * 21 + leapdays)
```
Run the property-based test again and… no errors! One interesting thing here is, the property-based test never changed, and yet it uncovered many errors. Also, while it might seem like there was a lot of code to make the property-based test, most of it is reusable. Another interesting thing is that, even though we only checked one simple property and focused on making that property work correctly, the final code that passed the property-based test does everything right. To understand how that’s possible we’ll dive into the test and explain all the different parts.

Here’s the code again:

```python
import datetime
import time
import random

from agecheck import twenty_first


# Our property-based test! It generates a lot of examples, and then it checks
# the property we've identified, being in the same month, for each of them.

def test_must_be_in_same_month():
  # Okay to check lots, because our assert will stop us at the first problem
  for ii in range(10000):
    birthday = random_day()
    error_message = """{} doesn't have the right twentyfirst birthday!
      Instead it has {}""".format(birthday, twenty_first(birthday))
    assert birthday.month == twenty_first(birthday).month, error_message


# Generators! These help us make what we'll use in our tests.
# Since our function takes birthdays, we need to make
# a generator for them.

def random_day():
  date_min = datetime.date(1970, 1, 1)
  date_max = datetime.date(2037, 12, 31)
  random_posix_time = random.randrange(to_posix(date_min), to_posix(date_max))
  return from_posix(random_posix_time)


def from_posix(seconds):
  return datetime.date.fromtimestamp(seconds)


def to_posix(date):
  return time.mktime(date.timetuple())


if __name__ == '__main__':
  test_must_be_in_same_month()
```
The test code looks a lot like any test, but there are some key differences that let it do a lot more than a typical test. Notice that the first thing in the test is a big loop. We aren’t testing one value, we’re testing ten thousand. Second, we aren’t picking what value we’re testing with, only the kind of value it is — a day. But once we have the value in our birthday variable, the rest looks just like we saw with our first attempt at testing the same month property with a normal unit test: `birthday.month == twenty_first(birthday).month`.

If we were using a property-based testing library, like we do in the rest of this book, it would handle running the test a bunch of times and generating the random values for us. It would also do other things, like “shrink” values that fail, looking for the simplest (and easiest to understand) failure cases. For some kinds of values, it would contain ready made ways to generate them, which brings us to the next part of our test: generating values.

In order to generate random days, we need to start with some random thing we can already generate. Luckily, the Python standard library already knows how to generate a random value from a range of numbers, and also how to turn a number of seconds into a date, so that’s what we do. If we were using hypothesis, the property-based testing library for the rest of the book, we would use the ready made generators it has for dates. In fact, the entire file would look like this, using hypothesis (and pytest to run the test):

```python

from hypothesis import given
from hypothesis.strategies import dates
from agecheck import twenty_first


@given(dates())
def test_must_be_in_same_month(birthday):
  error_message = """{} doesn't have the right twentyfirst birthday!
    Instead it has {}""".format(birthday, twenty_first(birthday))
  assert birthday.month == twenty_first(birthday).month, error_message
```
That’s much _shorter_ than our non-property-based tests, and helps us vastly more! Of course, this is an example I specifically chose to show off what property-based testing is capable of. In many cases you have to write your own generators, and that can get very complex. But once you’ve written them, you can reuse them across many tests, and the payoff from even a single property-based test can be enormous.

I hope the above helps explain why property-based tests can be very powerful, and helps expose how and why they work. While they can involve some up front work to write and adapt generators for the sorts of data your functions expect, once you have those generators the tests tend to be clean, and help you save your brainpower by acting as a better opponent to defeat your past self, instead of requiring you to outsmart your own code.

If you’re interested in more reading about property-based tests, the hypothesis [blog](http://hypothesis.works/articles/intro/) and [documentation](https://hypothesis.readthedocs.io/en/latest/) are excellent. For an example of using a more complicated generator to test data structure transformations, you can read my previous [property-based testing post](/post//muddled-property-based-tests/), which covers how to test some code I wrote in an effort to [explain Pythonic thinking](/post/messily-thinking-pythonically/).
