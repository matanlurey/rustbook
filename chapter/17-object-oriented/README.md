# Object Oriented Programming Features of Rust[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch17-01-what-is-oo.html>

> **Note**: There is no program included in this chapter, as it's notes only.

## Characteristics of Object-Oriented Langauges

_The Gang of Four_ defines OOP this way:

> Object-oriented programs are made up of objects. An _object_ packages both
> data and the procedures that operate on that data. The procedures are
> typically called _methods_ or _operations_.

Using this definition, Rust is object-oriented: structs and enums have _data_,
and `impl` blocks provide methods on structs and enums. Even though Rust doesn't
_call_ structs or enums objects, they provide the same functionality.

Another aspect associated with OOP is _encauspulation_; Rust provides `pub`:

```rs
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```

Yet another aspect is _inheritance_, gaining the parent object's data and
behavior without defining them again. With this definition, Rust is _not_
object-oriented. However, Rust has other tools where you might pick inheritance:

- Traits support _default_ implementations.
- Traits support _polymorphism_, i.e. common behavior.

```rs
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

When we use trait objects, Rust must use dynamic dispatch; which incurs a
runtime cost over static dispatch and disables inlining. However, we get extra
flexibility - so it's a tradeoff worth considering.

## Re-imagining the `State` pattern

A more Rust-y `State` pattern is using _types_ to encode states and behavior:

```rs
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

Note that `Post::new` creates a `DraftPost`, and only `Post` has a public
`content` method. It's now impossible to get around these (contract) constraints
without a compiler error.
