# dataframe

This was built as a learning project as I am fairly new to Rust, so please excuse the limited testing, the slightly nasty overuse of [Rc Cells](https://doc.rust-lang.org/std/rc/struct.Rc.html) to provide pointers to memory locations and general design patters.
I could have used something like the [polars dataframe](https://github.com/pola-rs/polars) but decided I would learn more about Rust, it's ecosystem and how it works in general by trying to build my own simple dataframe.

## Structure
### Cell
This is an element of a row and column that contains a value converted to one of the structs in the [AnyType enum](https://github.com/caljoshba/dataframe/blob/master/src/cell/types/datatypes.rs#L17-L37).

This structure itself is fairly simple and only really contains getters and setters as well as a reference to the row it resides in so you can easily return all the elements of the row, or navigate to another element in the row.

### Column
This contains a Vec and HashMap. The Vec contains a list of all the cells in the column and the HashMap contains aggregated values which could be used for easily returning a list of results for the equivalent of a where clause.

This implements a few things such as the mean of the column, rolling means (mean over x elements, calculated for each cell), calculating the difference to the previoud cell's value etc.

### Row
This contains a list of all the cells in the specific row across all columns, stored as weak references to avoid a cyclic dependency.
When initialised, each row is given an index value as well as a timestamp.

### DataFrame
This is the culmination of all the above. It contains vectors of the rows and columns as well as methods for adding rows and columns.

## Running the application
There isn't really anything to run other than the tests which you can do so ensuring you have the [rust toolchain installed](https://www.rust-lang.org/tools/install) and then running:

    cargo test --all

Or to run the tests inside docker

    docker-compose run --rm -T rusty cargo test

