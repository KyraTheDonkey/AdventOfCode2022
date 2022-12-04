# AdventOfCode2022
Hello, I'm a recent university graduate in Computer Science.
You might be here, browsing my github page for multiple reasons, but I cannot know which.

I took this opportunity to learn Rust. I have basically no prior knowledge of the language, and
my previous experience is all from a couple days where I read the [docs](https://doc.rust-lang.org/book).

I'm going to document more or less what I found on each day to keep track of what I figured out.
Anything I learnt that was interesting will be added, and my thoughts on the day, if any.

## Day 1
Learnt about the barebones I need to write the necessary code.
I already had Rust installed, and knew a bit about how Cargo worked, so setting up the initial part was fine.
I had to figure out arguments, file reading, and discovered how one would write neat code, with a main and a lib file.
Variable definitions were obviously a bit painful.

As to the problem itself, I originally just looped through everything and found the highest value,
but when I had to do part 2, I saw sorting would be much better, which I then employed to great effect.
I also made a [pull request](https://github.com/AzoraHusky/AdventOfCode2022/pull/1) 
on my friend's C++ code, since he was doing a lot of 
error checking and argument validation and I noticed one that he missed.

## Day 2
Inspired by my friend's validation on yesterday's code, I read up a bit deeper into the docs
at [12.3](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html)
to more neatly parse the arguments and program config, while validating that the part number
is indeed valid. Structs were used, with implementations on it, even though I'm pretty sure it's overkill with how
I implemented them.
I learnt about using a `match` as a switch statement, and not just for going through all the cases of an enum.

On the topic of enums, I did initially think of writing enums to handle the possible moves that I could read,
but eventually figured that the match statement would do well enough, and the enums were taking too long
to figure out. My code here is quite verbose with the two different parts,
but the important thing is I learnt about function pointers.

By [creating a new type](https://github.com/KyraTheDonkey/AdventOfCode2022/blob/1cbc506a643b3dd141700db4dfed2e7969119dfc/day2/src/lib.rs#L78),
I could use two full different functions, and just do a match between the two part numbers
to select which function I need to execute to get my scores back.

## Day 3
I'm just copy pasting my `main.rs` file for each day now, since it works well.
Today, I learnt about installing dependencies (`cargo add`), since I couldn't find
a way to create a slice, or substring with anything that was already in Rust.

I used a [inclusive range](https://github.com/KyraTheDonkey/AdventOfCode2022/blob/main/day3/src/lib.rs#L29-L30)
in a match statement, which is really cool. Trying to convert a character to a decimal ascii value
isn't clear, but makes sense once you know it.
The actual question wasn't too bad today, so pretty short.

## Day 4
Today's question again, just took a bit of problem solving, but wasn't too hard to code.
This time I did use some structs to store my data, with some added 
functions to better improve the readability of the code. My error messages
also had a bit more information, and I read up a bit more about RangeInclusive (`..=`)
to use in part 2 of the problem.
