# What is this repository?
To learn Rust based on [document](https://doc.rust-lang.org/book/title-page.html)

# Common Questions
Because the document is really helpful to understand programming, not only Rust itself, I summarize common questions which I make as if I were the biginner of programming, and answer the questions shortly. I added two types: R or G, which distinguish whether the question topic is about Rust itself or general programming after each question.

## Why do we use Enum? [G]
- Enums give you a way of saying a value is one of a possible set of values. For example, let's say we deal with IP addresses: ipv4 and ipv6. IP adress can take one of them, but **not both at the same time**. In this case, we would rather use Enum than structs.

## Why doesn't Rust have null? [R]
- Because null brings errors to our code easily. In some other languages, variables can always be null or not-null. However, when we assume a variable isn't null but it actually is, null error happens although null concept itself is useful. In Rust, instead of Null, you can use `Option<T>` which is one of Enums. Before you use `Option<T>`, you have to check if the variable is really present or absent. This can take advantage of null concept and at the same time it's safe to be used.
- ref: "Null References: The Billion Dollar Mistake,” Tony Hoare.

## Why do we want to use absolute path to call other module? [R]
- Because it's more likely we'll want to move our code definitions and item calls independently of each other

## Why do we specify the parent of a function and specify the full path of a struct? [R]
- When we use function, we care about the place in which it's built, and this is idiomatic. When we use struct, there is no strong reason to use the full path.

## What is the difference between array and collection? [G]
- Array and collection can contain multiple values, but the way to store data is different. Array stores data on the stack while collection stores data on the heap. Collection can grow or shrink at runtime.

## Why doesn't Rust allow us to index String like s[0]? [R]
- Because there is a difference between how human and computer deal with String. For example, let's say encoded UTF-8 "Hola", as a human the first letter should be H as same as a computer. Because "Hola"'s each character is stored using 1 byte per letter. However, in "Здравствуйте" case, as a human the first letter shoud be "З", but as a computer it's represented differently because Hindi is stored using 2 bytes per letter. The computer would return "208". The answer, then, is to prevent unexpected result and causing errors that might not be discovered immediately.

## When do we use panic and when not? [R]
- It depends on code calling a function which can have errors. If the code can recover from error, you can use `Result` and if not, you can use `panic`. For example, when you try to access an array past the end, `panic` should be called because it's insecure and it's considered impossible to recover unless you use `get` method. For other cases, when you create examples, prototype code and tests, you can use panic as a placeholder.

## Why doesn't Generic type affect runtime speed? [R]
- Because at compile Rust monomorphizes all generics, which means the compiler proceeds the reverse process to we make generics components. The compiler generate all components per each type.

## What is the difference among `FnOnce`, `FnMut`, `Fn`? [R]
- `FnOnce` is the most basic closure which is used or inherited every closure. You can use `FnOnce` when you want to `move` captured values only once. `FnMut` implements `FnOnce` and you can use it when your code don't satisfy `FnOnce` and you want to mutate captured values. Lastly, `Fn` implements `FnMut` and you can use it when your code don't satisfy `FnMut`.

## When to use rc and refcell? [R]
- `rc` are useful when you want to share ownership and don't want to make `let` for the first owner. In other words, unless every ownership is dropped, the value stays on the memory: called memory leak. `refcell` is usually used when to share a value using `rc` and want to change the value.