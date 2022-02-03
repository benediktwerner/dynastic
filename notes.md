## SQLite

https://blogs.gnome.org/jnelson/2015/01/06/sqlite-vacuum-and-auto_vacuum/
https://wiki.mozilla.org/Performance/Avoid_SQLite_In_Your_Next_Firefox_Feature
https://www.sqlite.org/undoredo.html

```
pragma application_id = 0x74737964  // "dyst", check when loading
pragma user_version = 1             // check when loading

pragma temp_store = memory
pragma journal_mode = wal
pragma synchronous = normal
pragma trusted_schema = off
pragma cell_size_check = on

pragma integrity_check
pragma quick_check  // like integrity_check but doesn't check UNIQUE and index constraings to run faster
pragma optimize     // recommended to run before closing the connection
pragma vacuum
```

https://diesel.rs/guides/getting-started.html
https://stackoverflow.com/questions/843780/store-boolean-value-in-sqlite
https://www.sqlitetutorial.net/sqlite-check-constraint/

https://github.com/justincy/d3-pedigree-examples
https://cs.brown.edu/people/rtamassi/gdhandbook/chapters/hierarchical.pdf

## Design

https://material.io/design/environment/elevation.html#default-elevations
https://stackoverflow.com/questions/36915508/what-are-the-material-design-dark-theme-colors
https://codepen.io/nana8/pen/VoXPKQ?editors=1100

Family Tree Maker: https://www.youtube.com/watch?v=NKOgr0upYUY
Roots Magic 8
Legacy 9

## TODO

- Add identifier back (for stuff like family search ids ig)
- How to handle transcribtions, translations, etc.
- Adjust attribution (allow list of contributors)
