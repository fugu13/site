---
title: 'Testing Algorithms for Data Structures: Binary Tree in Python'
date: '2023-12-13T16:20:00-08:00'
description: >
  What makes a satisfying test for in-place traversal of a binary tree? Using a Python
  example based on a common interview live coding question, see how Hypothesis can
  greatly increase confidence in code dealing with fundamental data structures.
---

In a recent programming interview, I was asked to implement in-place traversal for a
binary tree. I'm not a fan of this interview approach.

1. Live coding is not a realistic way of testing programming ability
2. Rubrics for live coding are frequently under-specified, so even when the original
   intent is that the exercise checks only for a base level of ability, interviewers
   in practice will assign significant value to modest variations in how candidates
   react to, again, a very unrealistic scenario
3. For data structure & algorithms specifically, the right way to implement
   them is working from, ideally, a written description of the algorithm and a
   known-working implementation, making them even less suitable for live coding

After the implementation part, with unfortunately little time remaining, the interviewer
asked a question I like a lot more: **how would you test the implementation?**

That's what this blog post explores.

![an example binary tree](.perseus/static/binarytree.png)


### A simple implementation

To start, here's a simple implementation using recursion. Our binary trees are generically
typed, and composed of nodes with a value and optional left and right children.

```python
from dataclasses import dataclass
from typing import TypeVar, Generic, Optional, Iterable

T = TypeVar('T')

@dataclass(frozen=True)
class Node(Generic[T]):
  value: T
  left: Optional['Node[T]'] = None
  right: Optional['Node[T]'] = None


def traverse_in_place_recursive(node: Node[T]) -> Iterable[Node[T]]:
  if node.left:
    yield from traverse_in_place_recursive(node.left)
  yield node
  if node.right:
    yield from traverse_in_place_recursive(node.right)
```

### A simple test

Now, how to test this? A typical approach would have a handful of unit tests that look
something like the one below.

```python
from tree.structure import Node
from tree.traverse import traverse_in_place_recursive


def test_simple_tree():
  tree = Node(1,
        Node(2,
           Node(3),
           Node(4,
              Node(6),
              None  # could be omitted but since it's an unbalanced node, being clear
           )
        ),
        Node(5)
      )
  assert [node.value for node in traverse_in_place_recursive(tree)] == [3, 2, 6, 4, 1, 5]
```

There might be several to cover whatever edge cases the author is particularly concerned,
but that's the basic model.

And maybe for an implementation this simple that feels good enough! But even simple
implementations get updated or replaced. As an example, even this simple implementation
might need to be replaced by one without recursion.

And most algorithm implementations are more complicated than this one, so even a lot of
individually tested handwritten examples don't feel very persuasive.

And even if every edge case for the existing implementation is covered, what if an updated
implementation has different edge cases?

### Tests with confidence

I've written before about [Property-based Testing](/post/muddled-property-based-tests/),
and that's what I recommend here. A typical test sets up an exact scenario, then checks
some aspect of the output, or even the entire output has an exact value.

A property-based test, on the other hand, describes the kinds of values that need to be
tested and a property--an invariant--that must hold for all those values. Then, the
testing framework generates bunches and bunches of those values, using strategies to
find ones that don't work for that property, and then simplifies the failing values
as much as possible to find a simple, easy to read counterexample.

Often, testing a handful of simple properties will provide high confidence that an
implementation is correct, because it will be hard to imagine an incorrect implementation
in good faith that also has those properties. 

Property-based testing is an excellent fit for data structures & algorithms questions,
because they generally possess strong, well-known properties.

### Generating

In order to perform property-based testing, the framework needs to know how to generate
the values being tested. Frameworks can automatically infer many kinds of values, so
sometimes no custom generation is required, and other times, like this one, the
values can be generated with a short custom generator, seen here.

```python
register_type_strategy(Node, recursive(
  builds(Node, uuids()),
  lambda nodes: builds(Node, uuids(), left=none() | nodes, right=none() | nodes),
))  # uuids so node values are always unique
```

This says, if we need to generate a `Node`, we do it with a recursive generator that
builds up the Node incrementally. First, it starts with a base case, which generates
Nodes that have no left or right, only UUID values. But it can also generate more Nodes
from other generated nodes, using the lambda function, where it can make a new node
with a UUID and left and right branches, either of which could be left empty (`None`) or
could contain any generated Node.

Or, to omit unnecessary details, the first argument of `recursive` says
"some Nodes look like this" and the second one says, "you can make Nodes that
contain other Nodes by combining them like this function says".

### A test that sounds complicated, but is simple

A very simple property for in-place traversal is, the number of items in
the traversal should be the same as the number of nodes in the tree. I'm using a
slight variant of that I often find useful here. Since I've constrained Node values to
be unique UUIDs, I can also check that the number of unique values in the in-place traversal
is the same as number of nodes in the tree.

First, here's one possible length implementation for Nodes.

```python
  def __len__(self) -> int:
    left_length = len(self.left) if self.left else 0
    right_length = len(self.right) if self.right else 0
    return 1 + left_length + right_length
```

Then, the test.

```python
@given(tree=...)
def test_correct_length(tree: Node):
  # using set to make sure we get the right number of unique values, which guards against
  # numerous possible issues
  values = [node.value for node in traverse_in_place_recursive(tree)]
  assert len(values) == len(set(values)) == len(tree)
```

(The `...` is not an omission, it tells Hypothesis to generate values based on the type
of the argument.)

That's a very short test! Even with the value generation code (which gets reused for
the other property-based test we'll be writing) and the comments, the total length is
about the same as the entire example unit test!

It isn't testing much, but it is definitely testing something. Specifically, it shows
that, so long as we believe the length implementation is correct (which we could test
separately), we're getting some traversal of the tree. We're getting a collection of unique values that correspond to the number of Nodes, and that's the definition of
a traversal.

Is it the right traversal? This test can't tell, but it is a traversal, and
that applies no matter what random tree we make, so we can be confident there's no edge
case that omits part of the tree.

Okay, now how do we gain confidence this is an in-place traversal?

### A more complicated test

So, imagine we've got our tree, and we pick any two nodes, randomly. Then, we find the
least common ancestor of the two nodes--the lowest node in the whole tree that's got
both of the nodes under it. By definition, since there's no lower part of the tree with
both of the nodes, one of the nodes must be on the left, and one of the nodes must be
on the right (ignoring for now the case where one of those nodes is an ancestor of the
other).

If we can write a test that captures that idea, we'll be really confident this is an
in-place traversal. There's one especially complicated part,
**find the least common ancestor**, but there's a nice trick to get around it.
Instead of picking two random nodes and then finding the least common ancestor,
we can pick any random node that is the root of a subtree of size at least two,
then pick one node from the left, and one node from the right!

And here's the complete code to do that.

```python
@given(tree=..., data=data())
def test_left_before_right(tree: Node, data: DataObject):
  # turn the full list of nodes, which we know has the right number, into a list
  # since we'll use it a few places
  everything = list(traverse_in_place_recursive(tree))

  # we're going to pick some subtree to work from, this might be the whole tree or it might be less
  starting_subtree = data.draw(sampled_from(everything))
  # the subtree has at least two nodes
  assume(len(starting_subtree) > 1)

  # take one value from the left and right each
  any_left = data.draw(sampled_from(list(traverse_in_place_recursive(starting_subtree.left)))) if starting_subtree.left else None
  any_right = data.draw(sampled_from(list(traverse_in_place_recursive(starting_subtree.right)))) if starting_subtree.right else None

  # verify that the value from the left is before the subtree node is before the value from the right,
  # with the value of the subtree root between them, in the full traversal
  if any_left and any_right:
    assert everything.index(any_left) < everything.index(starting_subtree) < everything.index(any_right)
  elif any_left:
    assert everything.index(any_left) < everything.index(starting_subtree)
  elif any_right:
    assert everything.index(starting_subtree) < everything.index(any_right)
```

It isn't as simple as the previous test, but without the comments it is still a little
shorter than the first example unit test!

Since it is a little more complicated, here's the line-by-line.

First, do the traversal we're going to test, and make it a list instead of a generator
since we'll need the values multiple times.

```python
everything = list(traverse_in_place(tree))
```

Second, use a special method, `data.draw()`, to derive a new generated value as any
random Node in the traversal, and make sure that subtree has at least 2 nodes. The assume
function means that, if it doesn't, Hypothesis will backtrack until it finds a test case
where the drawn value does.

```python
starting_subtree = data.draw(sampled_from(everything))
assume(len(starting_subtree) > 1)
```

Third, find one value on the left, and one value on the right. Either might be empty,
since we're covering the case where there might be a single child under a parent (the case
I said to ignore earlier), and if so, use None as a placeholder.

```python
any_left = data.draw(sampled_from(list(traverse_in_place(starting_subtree.left)))) if starting_subtree.left else None
any_right = data.draw(sampled_from(list(traverse_in_place(starting_subtree.right)))) if starting_subtree.right else None
```

Last, with cases for the different scenarios, make sure whatever values we have are in
the expected order in the original traversal.

```python
if any_left and any_right:
  assert everything.index(any_left) < everything.index(starting_subtree) < everything.index(any_right)
elif any_left:
  assert everything.index(any_left) < everything.index(starting_subtree)
elif any_right:
  assert everything.index(starting_subtree) < everything.index(any_right)
```

And with one test to verify the code does _some_ traversal, and one test to verify any
randomly selected nodes are in the expected order for an _in-place_ traversal, we
can be really confident that's what this does.

### Reimplementing

Okay, now time to make another implementation! We could run all these tests again for
that implementation, but there's an easier way. Imagine we've just written a new stack-based
implementation of in-place traversal. A complete test for the new implementation looks
like this.

```python
@given(tree=...)
def test_stack_same_as_recursive(tree: Node):
  assert list(traverse_in_place_stack(tree)) == list(traverse_in_place_recursive(tree))
```

After all, if the stack implementation, for a bunch of generated trees, results in the
same output as the recursive implementation, there we go!

### Conclusions

The property-based tests here include zero hand construction of test data,
zero need to know the algorithm implementation's edge cases
(the edge case discussed is an edge case in a test scenario for the data structure,
independent of the algorithm implementation), and zero logical duplication.

But hopefully you're feeling confident that any code that passes the tests really is
a working in-place traversal implementation!

And even more importantly, hopefully you're starting to imagine how a few short
property-based tests could increase your confidence that code dealing with data
structures and algorithms with well-defined correctness properties, actually has
those properties, even as the implementation changes.


### Complete files

`tree/structure.py`

```python
from dataclasses import dataclass
from typing import TypeVar, Generic, Optional

T = TypeVar('T')

@dataclass(frozen=True)
class Node(Generic[T]):
  value: T
  left: Optional['Node[T]'] = None
  right: Optional['Node[T]'] = None

  def __len__(self) -> int:
    left_length = len(self.left) if self.left else 0
    right_length = len(self.right) if self.right else 0
    return 1 + left_length + right_length
```

`tree/traverse.py`

```python
from typing import TypeVar, Iterable

from tree.structure import Node


T = TypeVar('T')


def traverse_in_place_recursive(node: Node[T]) -> Iterable[Node[T]]:
  if node.left:
    yield from traverse_in_place_recursive(node.left)
  yield node
  if node.right:
    yield from traverse_in_place_recursive(node.right)


def traverse_in_place_stack(node: Node[T]) -> Iterable[Node[T]]:
  visited = set()
  stack = [node]
  while stack:  # not empty
    top = stack.pop()  # the end of the stack, which is the top
    if top in visited:
      yield top
    else:
      visited.add(top)  # next time we see this, we want to just yield it
      # right gets visited last, so put it on stack first
      if top.right:
        stack.append(top.right)
      # we're not ready for top again until after left, so put it on next
      stack.append(top)
      # new top of the stack needs to be the left, which we handle next
      if top.left:
        stack.append(top.left)
```

`tests/test_in_place_traversal.py`

```python
from hypothesis import given, assume
from hypothesis.strategies import data, uuids, builds, register_type_strategy, none, recursive, DataObject, \
  sampled_from

from tree.structure import Node
from tree.traverse import traverse_in_place_recursive, traverse_in_place_stack


def test_simple_tree():
  tree = Node(1,
        Node(2,
           Node(3),
           Node(4,
              Node(6),
              None  # could be omitted but since it's an unbalanced node, being clear
           )
        ),
        Node(5)
      )
  assert [node.value for node in traverse_in_place_recursive(tree)] == [3, 2, 6, 4, 1, 5]


register_type_strategy(Node, recursive(
  builds(Node, uuids()),
  lambda nodes: builds(Node, uuids(), left=none() | nodes, right=none() | nodes),
))  # so node values are always unique


@given(tree=...)
def test_correct_length(tree: Node):
  # using set to make sure we get the right number of unique values, which guards against
  # numerous possible issues
  values = [node.value for node in traverse_in_place_recursive(tree)]
  assert len(values) == len(set(values)) == len(tree)


@given(tree=..., data=data())
def test_left_before_right(tree: Node, data: DataObject):
  # turn the full list of nodes, which we know has the right number, into a list
  # since we'll use it a few places
  everything = list(traverse_in_place_recursive(tree))

  # we're going to pick some subtree to work from, this might be the whole tree or it might be less
  starting_subtree = data.draw(sampled_from(everything))
  # the subtree has at least two nodes
  assume(len(starting_subtree) > 1)

  # take one value from the left and right each
  any_left = data.draw(sampled_from(list(traverse_in_place_recursive(starting_subtree.left)))) if starting_subtree.left else None
  any_right = data.draw(sampled_from(list(traverse_in_place_recursive(starting_subtree.right)))) if starting_subtree.right else None

  # verify that the value from the left is before the subtree node is before the value from the right,
  # with the value of the subtree root between them, in the full traversal
  if any_left and any_right:
    assert everything.index(any_left) < everything.index(starting_subtree) < everything.index(any_right)
  elif any_left:
    assert everything.index(any_left) < everything.index(starting_subtree)
  elif any_right:
    assert everything.index(starting_subtree) < everything.index(any_right)


@given(tree=...)
def test_stack_same_as_recursive(tree: Node):
  assert list(traverse_in_place_stack(tree)) == list(traverse_in_place_recursive(tree))
```