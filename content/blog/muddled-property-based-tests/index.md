---
title: Muddled (Property-based) Tests
date: '2018-02-09T12:12:12-08:00'
description: Writing really really good tests for a real world data transformation problem.
---

I solved a data transformation problem Pythonically, but there were no tests! Here’s how I might test it.

![copernican solar diagram](.perseus/static/spheres.png)

\_(This post originally appeared on Medium, but I've moved it to my personal site.)

In a previous post I solved a problem by [Messily Thinking Pythonically](/post/messily-thinking-pythonically/). There was one very Pythonic thing I didn’t do, though — I didn’t write **tests**. Partly that was because this was a quick bit of code I wrote one-off, and also partly because that blog post was already a bit over-long.

To recap, the core of the problem is simple: transform data from structures like

```python
boop = {
    “hello”: [1, 2, 3],
    “bye”: [4, 5, 6]
}
```

Into structures like

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

You can review the particular solution I came up with, but this post doesn’t rely on the details. All we need for now is to assume there’s a function combos that does the transformation in full.

Okay, so, testing that. The obvious first test is:

```python
def test_combos():
    boop = …
    beep = …
    assert combos(boop) == beep
```

Not too bad, but **only tests one very particular dataset**, there should probably be some other examples… how many? Also, while this particular transformation code doesn’t have many conditionals, so there probably aren’t too many edge cases… **what are the edge cases**? And what if this code did have a number of conditionals? **When will we have tested enough to have confidence that this code works across anything we’re likely to throw at it, not just a few examples?**

That’s where **property-based testing** comes in, a testing technique I’m a big advocate of. Unlike unit testing or integration testing, which are names related to the scale of code being tested (sort of), property-based testing is about how the testing is done, and can be used at any testing scale. The key differences are, in property-based testing you don’t choose specific data to work with, and the things you test for are **properties that hold true** no matter what data you use. If you’re running the code along with, go ahead and create a virtual environment (I recommend using [pipenv](https://docs.pipenv.org), so make a folder, then inside it run pipenv install --three) and install both pytest and hypothesis (pipenv install --dev pytest hypothesis). If you put the tests in a file that starts with the name test\_ in the same folder with the code you’re testing, then run pytest there (pipenv run pytest), it’ll run your tests.

But what’s a property-based test? First, here’s an example (without boilerplate):

```python
def test_right_number_products(example):
    lengths = [len(values) for values in example.values()]
    number_expected = reduce(mul, lengths, 1) # product of all lengths
    transformed = combos(example)
    assert len(transformed) == number_expected
```

Okay, so this test takes in some example data (don’t worry about that yet, except to know it will be data structured like in “boop”), then it finds how long each of the value lists is, then it multiplies those numbers together, and checks that the number of values in the output of combos(example) is the same as the product. Stepping back a little, this is a property any correctly implemented combos function must have, no matter what data is provided, since a list of dictionaries containing the products of all category values must have the same number of dictionaries as there are products (seems almost silly to explain it in that much detail, right? Good property-based tests are often about finding those obviously-correct things).

Where did the example come from, though? The testing framework, hypothesis, makes it. To explain how, here’s the boilerplate:

```python
from hypothesis import given
from hypothesis.strategies import text, lists, integers, floats, one_of, dictionaries

keys = text()
values = one_of(integers(), floats())

categories = dictionaries(keys, lists(values, min_size=1, max_size=10, unique=True), min_size=1, max_size=4)

@given(categories)
def test_right_number_products(example):
    …
```

Start from the bottom. @given(categories) tells hypothesis this is a test to generate values for, and it should make the argument’s values using the generator named categories.

The generator named categories is a generator for dictionaries (imported), and it makes keys using a generator named keys. It makes values by taking a generator for lists (imported) and using it with a generator named values to make lists of length 1 to 10, where every value in the list is unique. The dictionary made will have 1 to 4 keys. (The length restrictions here are because we’re dealing in products, so runtimes can get very long if we don’t have maximums).

The generator named values generates, each time it is asked to make a new value, either an integer or a float.

The generator named keys generates text.

Every time hypothesis goes to generate values for the test, it will make values that fit all those rules. This generator isn’t very complicated to build with the generators hypothesis already provides, but in a worst case scenario you can completely define your own.

Seems like a lot of work compared to just having some examples, maybe? But it isn’t too bad to write. And sure, we need that number of values, but they need to be pretty specific values, not just any values.

Luckily, working up a generator for values like you need comes with dividends: you can keep using it. Three more property-based tests:

```python
@given(categories)
def test_all_values_unique(example):
    transformed = combos(example)
    uniques = frozenset(tuple(value.items()) for value in transformed)
    assert len(transformed) == len(uniques)

@given(categories)
def test_all_values_present(example):
    transformed = combos(example)
    for key, expected_values in example.items():
        transformed_values = frozenset(item[key] for item in transformed)
        assert frozenset(expected_values) == transformed_values

@given(categories)
def test_all_keys_present(example):
    transformed = combos(example)
    for product_dictionary in transformed:
        assert product_dictionary.keys() == example.keys()
```

With these tests, we verify that, in addition to having the right number of dictionaries in the list from the first test, every dictionary in the list is unique, all values for each original key are present as values of that key in the output (and vice versa), and every dictionary in the list has the same keys as the original dictionary. What gets really interesting is, we’re starting to reach a place where **it is hard to imagine any code that passes those tests and is not correct**. That’s powerful! A small number of property-based can be **more readable**, **more verifiable**, and instill **more confidence** code works than a much larger number of unit tests with numerous handwritten example values.

In this case, we could jam all those things into one test, since they all involve first taking an example, then computing the combos, then checking some property, but that’d be messy, and this is clean. Thinking should be (will be) messy, but** code should be clean**.

This post is less about how I made the tests and more about explicating their structure, compared to the post about the original problem, but I hope it has enough explanation that property-based tests start making sense. Good property-based tests can take more work to create up front, but pay off considerably. I also didn’t get into how good tests can drive the development process, since in this case they didn’t, but I think that can matter less for property-based tests. They suffer much less from the tendency of tests-written-after to overly involve implementation knowledge, due to the **focus on universal properties**.

Here’s the complete original code plus the full tests:

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

```python
from categories import combos

from functools import reduce
from operator import mul

from hypothesis import given
from hypothesis.strategies import text, lists, integers, floats, one_of, dictionaries

keys = text()
values = one_of(integers(), floats())

categories = dictionaries(keys, lists(values, min_size=1, max_size=10, unique=True), min_size=1, max_size=4)


@given(categories)
def test_right_number_products(example):
    lengths = [len(values) for values in example.values()]
    number_expected = reduce(mul, lengths, 1)  # product of all lengths
    transformed = combos(example)
    assert len(transformed) == number_expected


@given(categories)
def test_all_values_unique(example):
    transformed = combos(example)
    uniques = frozenset(tuple(value.items()) for value in transformed)
    assert len(transformed) == len(uniques)


@given(categories)
def test_all_values_present(example):
    transformed = combos(example)
    for key, expected_values in example.items():
        transformed_values = frozenset(item[key] for item in transformed)
        assert frozenset(expected_values) == transformed_values


@given(categories)
def test_all_keys_present(example):
    transformed = combos(example)
    for product_dictionary in transformed:
        assert product_dictionary.keys() == example.keys()
```