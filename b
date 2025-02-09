   Compiling hvr v0.1.0 (/home/all/repos/hvr)
warning: unused import: `x11rb::protocol::Event as X11Event`
 --> src/main.rs:5:5
  |
5 | use x11rb::protocol::Event as X11Event;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0107]: struct takes 0 generic arguments but 2 generic arguments were supplied
  --> src/main.rs:74:13
   |
74 |     layout: Layout<12, 4>,
   |             ^^^^^^------- help: remove these generics
   |             |
   |             expected 0 generic arguments
   |
note: struct defined here, with 0 generic parameters
  --> /home/half-arch/.cargo/registry/src/index.crates.io-6f17d22bba15001f/keyberon-0.1.1/src/layout.rs:20:12
   |
20 | pub struct Layout {
   |            ^^^^^^

error[E0747]: constant provided when a type was expected
  --> src/main.rs:75:20
   |
75 |     matrix: Matrix<12, 4>,
   |                    ^^

error[E0277]: `{integer}` is not an iterator
  --> src/main.rs:84:37
   |
84 |             matrix: Matrix::new(12, 4).expect("Failed to initialize matrix"),
   |                     -----------     ^ `{integer}` is not an iterator
   |                     |
   |                     required by a bound introduced by this call
   |
   = help: the trait `Iterator` is not implemented for `{integer}`, which is required by `for<'a> &'a mut {integer}: Iterator`
   = note: required for `&'a mut {integer}` to implement `for<'a> Iterator`
note: required by a bound in `Matrix::<C, R>::new`
  --> /home/half-arch/.cargo/registry/src/index.crates.io-6f17d22bba15001f/keyberon-0.1.1/src/matrix.rs:51:41
   |
49 |     pub fn new<E>(cols: C, rows: R) -> Result<Self, E>
   |            --- required by a bound in this associated function
50 |     where
51 |         for<'a> &'a mut R: IntoIterator<Item = &'a mut dyn OutputPin<Error = E>>,
   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Matrix::<C, R>::new`

Some errors have detailed explanations: E0107, E0277, E0747.
For more information about an error, try `rustc --explain E0107`.
warning: `hvr` (bin "hvr") generated 1 warning
error: could not compile `hvr` (bin "hvr") due to 3 previous errors; 1 warning emitted
