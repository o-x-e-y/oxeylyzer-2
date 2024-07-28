---
title: "How Oxeylyzer Works"
date: 2024-7-28
description: "Taking a closer look at Oxeylyzer's internals"
---

# How Oxeylyzer Works

Taking a closer look at a keyboard layout analyzer's internals.

## Introduction

Oxeylyzer 2 is a keyboard layout analyzer written in Rust, and it's also the analyzer powering the
stats on this website. It was made with strong error handling, wasm compliance and generation speed
in mind, the former two version one lacks. My goal in this article is to explain what it takes to
go from a corpus text and a layout to different stats on the screen, and what goes into
optimizing layout generation.

## Processing corpus files

The first step any analyzer must be able to do is process text files into ngram data. The way
oxeylyzer handles this is by iterating over trigrams in a blob of text and tallying up each
character, bigram, skipgram and the trigram itself like this:

```rust
fn generate_data(iter: impl Iterator<Item = char>) -> IntermediateData {
    let mut data = IntermediateData::default();
    let mut iter = iter.into_iter();

    if let Some(mut c1) = iter.next() {
        data.add_char(c1)

        if let Some(mut c2) = iter.next() {
            data.add_char(c2);
            data.add_bigram(c1, c2);

            for c3 in iter {
                
                data.add_char(c3);
                data.add_bigram(c2, c3);
                data.add_skipgram(c1, c3);
                data.add_trigram(c1, c2, c3);

                c1 = c2;
                c2 = c3;
            }
        }
    }

    data
}
```

where each of these `add_*` functions looks something like this, adding one to the total or
inserting one if the ngram wasn't encountered before.

```rust
fn add_bigram(&mut self, c1: char, c2: char) {
    self.bigrams
        .entry([c1, c2])
        .and_modify(|f| *f += 1)
        .or_insert(1);
}
```

Once this is done, every ngram is summed up and every value is divided by that total, after which
the data is ready to be saved as json. That is the gist at least - (almost) every analyzer does
some sort of cleaning step before generating the data to be able to do things like inserting shift
presses and removing unknown characters.

## Layouts

Once we have the data, we can load in some layouts. A very simple way to represent a keyboard
layout is one similar to the one Oxeylyzer v1 uses:

```rust
pub struct Layout {
    keys: [char; 30]>,
    positions: HashMap<char, usize>
}
```

This allows us to grab characters from known indices, or return key positions when we only have
access to the characters themselves. We have no thumb keys in this setup, just 4 index columns and
6 columns for our middle, ring and pinky fingers.

## Analysis

Now the real fun begins. With our data and a layout, we can go ahead and write some code that takes
both of these and spits out a value.

### Sfbs

A very simple stat to check is sfbs. Step one is actually the hardest part, where we have to take
each column on the layout and get every combination of 2 key pairs. The `tuple_combinations`
function from the [`Itertools`](https://crates.io/crates/itertools) library is very useful for
this. In terms of writing anything but display code it's actually easier to say that the layout
consists of 6 columns and then 2 index columns, but this example has every key in its visually
sensible position:

```rust
fn sfb_indices() -> [(usize, usize); 48] {
    (0..3)
        .into_iter()
        .flat_map(|i| {
            // left pinky, ring and middle
            [i, i+10, i+20]
                .into_iter()
                .tuple_combinations::<(_, _)>()
        })
        .chain([3, 5].into_iter().flat_map(|i| {
            // left and right index columns
            [i, i+1, i+10, i+11, i+20, i+21]
                .into_iter()
                .tuple_combinations::<(_, _)>()
        }))
        .chain((7..10).into_iter().flat_map(|i| {
            // right middle, ring and pinky
            [i, i+10, i+20]
                .into_iter()
                .tuple_combinations::<(_, _)>()
        }))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
```

Before we go ahead and analyze a layout with these indices, it's a good idea to create some sort of
overarching analysis struct to store things like this in a central place. We can also keep our
data there:

```rust
pub struct Analyzer {
    sfb_indices: [(usize, usize); 48]
    data: Data,
}
```

From here, we can create a member function that loops over these indices to find an sfb value. Note
that we take both the given sfb and the reverse!

```rust
pub fn sfbs(&self, layout: &Layout) -> f64 {
    self
        .sfb_indices
        .iter()
        .map(|i1, i2| {
            let bigram1 = (layout.char(i1), layout.char(i2));
            let bigram2 = (layout.char(i2), layout.char(i1));

            let sfb1 = self.data.get_bigram(bigram1);
            let sfb2 = self.data.get_bigram(bigram2);

            sfb1 + sfb2
        })
        .sum()
}
```

This code can be reused to get the amount of sfs as well. In fact, you can follow the same process
of creating a set of indices and looping over them to get any sort of bigram% you want: lsbs, scissors and anything else are within reach from here.

### Trigrams

<img src="/public/images/iandof.png"/>