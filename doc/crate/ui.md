UI
===

Great read about rust guis in 2022:
 https://raphlinus.github.io/rust/gui/2022/07/15/next-dozen-guis.html.
Generally his articles look informative.

The following is mostly word of mouth, but I did go through the libraries for my own impression.
Up to date as of 2022-08.


druid
---

Good architecture but hard to get started & not really intended for widespread use.
Maintainer runs the blog above and clearly cares about the quality.
Keep an eye on it!

tauri
---

Big & complex, with a rust layer and display layer.
Similar to r-sat, message passing & such.
Doesn't need wasm.

yew
---

Friendly; looks like a wasm framework.
Probably too finicky to use early.

iced
---

Elm-based architecture.
Looks nifty, looks rust only is possible.
Go with this early on, probably.

egui
---

Easy, but immediate mode only.
Probably good for simple dev ui.

azul
---

Rust, C, C++.
Looks like they do it in pure rust, but it's writing html & css (verbosely).

slint
---

Good-looking, looks easy enough to work with.
Not text focused, more generic.

conrad
---

Unmaintained.

relm
---

Elm-based, but low maintained.

sycamore
---

Another with pure rust & verbose, reactive html.
Nice-looking tutorials & could be good!

rui
---

Single maintainer, very active, but unused otherwise afaik.
