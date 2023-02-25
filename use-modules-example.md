Directory tree:
```
src
├── lib.rs
├── main.rs
└── to_do
    ├── mod.rs
    └── structs
        ├── base.rs
        ├── done.rs
        ├── mod.rs
        ├── pending.rs
        └── traits <-- How do you allow these files to be accessed in the rest of the crate?
            ├── create.rs
            ├── delete.rs
            ├── edit.rs
            ├── get.rs
            └── mod.rs
```
How do you allow these files to be accessed to the rest of the crate?

```
traits/
    ├── create.rs
    ├── delete.rs
    ├── edit.rs
    ├── get.rs
    └── mod.rs
```
You expose them with `pub mod <file_name>` without the `.rs` extension

`traits/mod.rs`
```
pub mod create; // --> create.rs
pub mod delete; // --> delete.rs
pub mod edit; // --> edit.rs
pub mod get; // --> get.rs
```
Now, that they are accessible, we have to make them accessible to the structs by publicly defining them in:

 `structs/mod.rs`
```
└── structs/
    ├── base.rs
    ├── done.rs
    ├── mod.rs <-- this one
    ├── pending.rs
    └── traits
```

mod.rs
```
pub mod traits;
mod base
```