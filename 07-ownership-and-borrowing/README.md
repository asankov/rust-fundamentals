# 07. Ownership and Borrowing

The most important topic in this course - the way Rust does memory management.

Important questions:

- Who owns the data?
- Pass by value or pass by reference?
- Is the data mutable?

> [!IMPORTANT]
> Ownership and Borrowing only apply to data on the heap.

In Rust there can only be only one owner of data at time for a given memory location.
