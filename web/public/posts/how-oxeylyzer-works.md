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
fn generate_data(text: &str) -> IntermediateData {
    let mut data = IntermediateData::default();
    let mut iter = text.chars();

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
layout is one similar to the one Oxeylyzer v1 uses. Every row is 10 characters wide, with three
rows total:

```rust
pub struct Layout {
    keys: [char; 30]>,
    positions: HashMap<char, usize>
}
```

This allows us to grab characters from known indices, or return key positions when we only have
access to the characters themselves. We have no thumb keys in this setup, just 4 index columns and
6 columns for our middle, ring and pinky fingers. All we need to load such a layout is 30
characters in some form.

## Analysis

Now the real fun begins. With our data and a layout, we can go ahead and write some code that takes
both of these and spits out a value.

### Bigrams

A very common stat to check is sfbs. Step one is actually the hardest part, where we have to take
each column on the layout and get every combination of 2 key pairs. The `tuple_combinations`
function from the [`Itertools`](https://crates.io/crates/itertools) library is very useful for
this.

```rust
fn sfb_indices() -> [(usize, usize); 48] {
    // left pinky, ring and middle
    (0..3)
        .into_iter()
        .flat_map(|i| {
            // 3 keys, one on each row
            [i, i+10, i+20]
                .into_iter()
                .tuple_combinations::<(_, _)>()
        })
        // left and right index columns
        .chain([3, 5].into_iter().flat_map(|i| {
            // 6 keys spanning 3 rows and 2 columns
            [i, i+1, i+10, i+11, i+20, i+21]
                .into_iter()
                .tuple_combinations::<(_, _)>()
        }))
        // right middle, ring and pinky
        .chain((7..10).into_iter().flat_map(|i| {
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
    data: Data,
    sfb_indices: [(usize, usize); 48]
}
```

From here, we can create a member function that loops over these indices to find an sfb value. Note
that we take both the given sfb indices, but also the reverse!

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
of creating a set of indices and looping over them to get any sort of bigram% you want: lsbs,
scissors and anything else are within reach from here.

### Trigrams

Trigrams create a new set of challenges, though the biggest problems only rear their head when
writing code to generate layouts. Trigram stats are done between fingers and not individual key
positions, so the strategy here is to go from trigram -> 3 indices -> 3 fingers -> trigram type.

Step one is easy enough: simply extend our `Layout` to map three characters to three indices:

```rust
fn trigram_positions(&self, [c1, c2, c3]: [char; 3]) -> Option<[usize; 3]> {
    let p1 = self.positions.get(&c1);
    let p2 = self.positions.get(&c3);
    let p3 = self.positions.get(&c3);

    match (p1, p2, p3) {
        (Some(p1), Some(p2), Some(p3)) => Some((p1, p2, p3)),
        _ => None
    }
}
```

We check if each character on the layout exists, and if they do, we return the indices corresponding
to these characters. Step two is also very easy: we know the shape of our keyboard, so we can just
have a predefined array to go from one to the other. We represent each finger as a number between
0 and 7, which we can do because we ignore thumbs:

```rust
pub const POS_FINGERS: [usize; 30] = [
    0, 1, 2, 3, 3,  4, 4, 5, 6, 7,
    0, 1, 2, 3, 3,  4, 4, 5, 6, 7,
    0, 1, 2, 3, 3,  4, 4, 5, 6, 7,
];
```

We can also add this to the analysis struct. Now we have three fingers we want to get a
corresponding trigram. Considering the fact we have three integers, the strategy here is to populate
an array first to only have to index into it for every trigram. The way to do this is by going over
each finger combinations, and writing some code to determine what trigram it is. Rolls will be the
example here. Rolls are defined as trigrams where one key is one hand, and two are on the other. To
check if something is a roll we can do something like this:

```rust
fn is_roll([f1, f2, f3]: [usize; 3]) -> bool {
    // first two on left hand, second on right hand
    f1 < 4 && f2 < 4 && f3 >= 4 ||
        // first on left hand, second two on right hand
        f1 < 4 && f2 >= 4 && f3 >= 4 ||
        // first on right hand, second two on left hand
        f1 >= 4 && f2 < 4 && f3 < 4 ||
        // first two on right hand, second on left hand
        f1 >= 4 && f2 >= 4 && f3 < 4
}
```

We can do this for redirects, alternation and any other trigam type. With these made, we can
populate the array like this:

```rust
pub enum TrigramType {
    Roll,
    Redirect,
    Unknown,
}

impl TrigramType {
    pub fn get(fingers: [usize; 3]) -> Self {
        if is_roll(fingers) {
            return Self::Roll
        }

        if is_redirect(fingers) {
            return Self::Redirect
        }

        Self::Unknown
    }
}

// We have 8 * 8 * 8 = 512 possible combinations
fn get_trigram_types() -> [TrigramType; 512] {
    let mut res = [TrigramType::Unknown; 512];

    for f1 in 0..8 {
        for f2 in 0..8 {
            for f3 in 0..8 {
                let index = f1 * 64 + f2 * 8 + f3;

                res[index] = TrigramType::get([f1, f2, f3]);
            }
        }
    }

    res
}
```

We can add this as a field to our analyzer struct. Now we can loop over all trigrams in our data and
tally up the results in an intermediate struct.

As far as analyzing a layout goes, this is really all there is to it. There are some things we can
optimize here however. The main slowdown we can get ouf the way is the char -> position map. Instead
of actual characters we could map them to integers and use an array, which speeds things up a _lot_.

Thanks for reading cutie patooties

<img src="/public/images/iandof.png" />