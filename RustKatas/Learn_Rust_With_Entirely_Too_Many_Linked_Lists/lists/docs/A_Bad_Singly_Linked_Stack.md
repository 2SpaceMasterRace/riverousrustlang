# Basic Data Layout

Alright, so what's a linked list? Well basically, it's a bunch of pieces of data on the heap (hush, kernel people!) that point to each other in sequence. Linked lists are something procedural programmers shouldn't touch with a 10-foot pole, and what functional programmers use for everything.


It seems fair, then, that we should ask functional programmers for the definition of a linked list:

"A List is either Empty or an Element followed by a List". This is a recursive definition expressed as a sum type, which is a fancy name for "a type that can have different values which may be different types". Rust calls sum types enums!


Consider a list with two elements:

[] = Stack
() = Heap

[Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)

There are two key issues:

    We're allocating a node that just says "I'm not actually a Node"
    One of our nodes isn't heap-allocated at all.

On the surface, these two seem to cancel each-other out. We heap-allocate an extra node, but one of our nodes doesn't need to be heap-allocated at all. However, consider the following potential layout for our list:

[ptr] -> (Elem A, ptr) -> (Elem B, *null*)

In this layout we now unconditionally heap allocate our nodes. The key difference is the absence of the junk from our first layout. What is this junk? To understand that, we'll need to look at how an enum is laid out in memory.


In general, if we have an enum like:

enum Foo {
    D1(T1),
    D2(T2),
    ...
    Dn(Tn),
}

A Foo will need to store some integer to indicate which variant of the enum it represents (D1, D2, .. Dn). This is the tag of the enum. It will also need enough space to store the largest of T1, T2, .. Tn (plus some extra space to satisfy alignment requirements).

The big takeaway here is that even though Empty is a single bit of information, it necessarily consumes enough space for a pointer and an element, because it has to be ready to become an Elem at any time. Therefore the first layout heap allocates an extra element that's just full of junk, consuming a bit more space than the second layout.

One of our nodes not being allocated at all is also, perhaps surprisingly, worse than always allocating it. This is because it gives us a non-uniform node layout. This doesn't have much of an appreciable effect on pushing and popping nodes, but it does have an effect on splitting and merging lists.


Consider splitting a list in both layouts:

layout 1:

[Elem A, ptr] -> (Elem B, ptr) -> (Elem C, ptr) -> (Empty *junk*)

split off C:

[Elem A, ptr] -> (Elem B, ptr) -> (Empty *junk*)
[Elem C, ptr] -> (Empty *junk*)

layout 2:

[ptr] -> (Elem A, ptr) -> (Elem B, ptr) -> (Elem C, *null*)

split off C:

[ptr] -> (Elem A, ptr) -> (Elem B, *null*)
[ptr] -> (Elem C, *null*)

Layout 2's split involves just copying B's pointer to the stack and nulling the old value out. Layout 1 ultimately does the same thing, but also has to copy C from the heap to the stack. Merging is the same process in reverse.

One of the few nice things about a linked list is that you can construct the element in the node itself, and then freely shuffle it around lists without ever moving it. You just fiddle with pointers and stuff gets "moved". Layout 1 trashes this property.


Alright, I'm reasonably convinced Layout 1 is bad. How do we rewrite our List? Well, we could do something like:

pub enum List {
    Empty,
    ElemThenEmpty(i32),
    ElemThenNotEmpty(i32, Box<List>),
}

Hopefully this seems like an even worse idea to you. Most notably, this really complicates our logic, because there is now a completely invalid state: ElemThenNotEmpty(0, Box(Empty)). It also still suffers from non-uniformly allocating our elements.

We previously saw that every enum has to store a tag to specify which variant of the enum its bits represent. However, if we have a special kind of enum:

enum Foo {
    A,
    B(ContainsANonNullPtr),
}

the null pointer optimization kicks in, which eliminates the space needed for the tag. If the variant is A, the whole enum is set to all 0's. Otherwise, the variant is B. This works because B can never be all 0's, since it contains a non-zero pointer. Slick!

Can you think of other enums and types that could do this kind of optimization? There's actually a lot! This is why Rust leaves enum layout totally unspecified. There are a few more complicated enum layout optimizations that Rust will do for us, but the null pointer one is definitely the most important! It means &, &mut, Box, Rc, Arc, Vec, and several other important types in Rust have no overhead when put in an Option! (We'll get to most of these in due time.)
