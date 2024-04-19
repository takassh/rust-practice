# What is this?
- the repository to make simple one shot channel using atomic and memory ordering

## Words
- MeybeUninit
    - low level for `Option`
- unsafe
    - wrapped by unsafe means it can make undefined error like data race
- Pass by value
    - If you want to call method only once, you can use pass by value
- PhantomData
    - it can be used for some reasons. In this repository, it disallows struct to be Send