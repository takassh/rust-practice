# What is this?
- To learn dependency injection (DI) with Rust
- This repository addresses two DIs: static and dynamic dispatch

## How to make static dispath?
1. define a trait which represents interfaces of the implementation.
    - This is `trait Repository` in the code
1. implement the trait
    - This is `impl Repository for RepositoryImpl` in the code
1. have the implementation via the trait using Generics
    - This is `pub struct Service<R: Repository>` in the code
1. resolve dependency using a struct and use the struct
    - This is `struct Module` and `fn main` in the code

## How to make dynamic dispatch?
It's almost same as static dispatch
1. define a trait which represents interfaces of the implementation.
    - This is `trait Repository` in the code
1. implement the trait
    - This is `impl Repository for RepositoryImpl` in the code
1. have the implementation via the trait using trait object
    - This is `pub struct Service` in the code. Instead of Generics like static one, contain `dyn Reposity` on the field.
1. resolve dependency using a struct and use the struct
    - This is `struct Module` and `fn main` in the code

## Which one do you should use?
- When you really care about runtime cost, you should use static dispatch because it's no cost during runtime, otherwise you can use dynamic dispatch because it's less boiler code than static one.
[Performance of Code Using Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics)
> You might be wondering whether there is a runtime cost when using generic type parameters. The good news is that using generic types won't make your program run any slower than it would with concrete types.

This is also good for you to undertand the difference [Trait Objects Perform Dynamic Dispatch](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch)