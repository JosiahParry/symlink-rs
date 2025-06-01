**NOTE** this is a fork of [symlink](gitlab.com/chris-morgan/symlink). There are not changes to this crate. But a fork is required as there has not been a published update in 8 years and this crate is exceptionally useful!

# `symlink-rs`: create (and delete) symlinks in a cross-platform manner

Rust’s standard library exposes platform-specific ways to create symlinks:

- On Windows, `std::os::windows::fs::{symlink_file, symlink_dir}` (because Windows does file and directory symlinks differently);
- On Unix platforms, `std::os::unix::fs::symlink` (because they don’t care about whether it’s a file or a directory).

There’s also `std::fs::soft_link`, deprecated because of the whole Windows situation, but potentially still useful: as I write, at the start of 2022, it’s the only stable way to create a symlink on the wasm32-wasi target (`std::os::wasi::fs::symlink_path` not yet being stable).

The situation is similar when removing symlinks: on most platforms all symlinks are files and must be removed with `std::fs::remove_file`, but on Windows directory symlinks must be removed with `std::fs::remove_dir` instead.

This is all a pain: as soon as you touch symlinks for Unix you need to add in lots of `#[cfg]` branches and other such messy things, or else lose Windows support for no good reason, or use a deprecated function that makes directory symlinks not work on Windows.

Enter the `symlink` crate. This crate gives you six cross-platform functions instead:

- `symlink_file`, which creates a file symlink on Windows and a common-or-garden symlink on other platforms;
- `symlink_dir`, which creates a directory symlink on Windows and a perfectly ordinary symlink on other platforms;
- `symlink_auto`, which creates a file or directory symlink on Windows, depending on an examination of the destination, and a perfectly ordinary symlink on other platforms;
- `remove_symlink_file`, which removes a file symlink on Windows and a common-or-garden symlink on other platforms;
- `remove_symlink_dir`, which removes a directory symlink on Windows and a perfectly ordinary symlink on other platforms;
- `remove_symlink_auto`, which removes a file or directory symlink on Windows, depending on an examination of the path, and a perfectly ordinary symlink on other platforms.

Back on the topic of `std::fs::soft_link`: it got deprecated in Rust 1.1.0 because it just created a file symlink on Windows, which is often wrong. `symlink_auto` creates a file *or* directory symlink, depending on what the target is. (But it’s also more fragile: unlike `symlink_file` and `symlink_dir`, it returns an error if the destination doesn’t exist or can’t be statted.)

And before this crate there was no good way to delete a symlink at all on Windows. Who knows, perhaps windows_file_type_ext will be stabilised eventually. But until then, there’s this crate.

## Best practices

You should generally avoid `symlink_auto` and `remove_symlink_auto`, preferring to use the more specific `symlink_file` or `symlink_dir` and `remove_symlink_file` or `remove_symlink_dir`, whichever seems appropriate for what you’re doing. (In real life you almost always know whether you’re making a file or a directory symlink, so say it in the code!)

**Make sure you use absolute paths for the destination.** I haven’t tested whether relative paths are treated consistently across platforms yet (whether they’re relative to the working directory or the symlink source path). TODO!

## Caution: symlinks are still less reliable on Windows

You can only reliably use symlinks from the Windows 10 Creators Update (mid-2017) onwards.

Before that, manipulating symlinks required a special privilege which practically meant you had to run a program as admin to get it to work. And symlinks were new to Vista; XP and older didn’t support them.

## My goal: integration with Rust

I would like to merge this into libstd in some form, because the symlink manipulation support in the standard library at present is hopeless for cross-platformness. I haven’t written an RFC yet; it should definitely start as a separate crate (that’s what this is). Here are some of my thoughts:

**Concerning `symlink_auto`**: it’s deliberately not named `symlink`; my hope is that people won’t just reach for it blindly but will think about what they are doing. A few things can happen to it (in my order of preference):

1. It can not exist. It’s really not *necessary*, and letting people be lazy isn’t always good. Encourage cross-platformness!
2. It can exist as `std::fs::symlink_auto`. The distinction is thus clear.
3. `std::fs::soft_link` can be undeprecated, with a change to its Windows semantics from “make a file symlink” to “make a file or directory symlink as appropriate, yielding an error if the destination doesn’t stat”.
4. `std::fs::soft_link` can be undeprecated, with a change to its Windows semantics from “make a file symlink” to “make a file or directory symlink as appropriate, going with a file symlink if the destination doesn’t stat”.
5. It can exist as `std::fs::symlink`. This is the obvious name, but as mentioned earlier encourages inefficient imprecision for Windows.

**Concerning `symlink_dir` and `symlink_file`**:

1. `std::fs::{symlink_file, symlink_dir}`, matching `symlink_auto` or nothing.

2. `std::fs::{soft_link_file, soft_link_dir}`, matching `soft_link` if it is undeprecated. But I don’t like the name “soft link,” anyway: no one calls them that, we all call them symlinks.

Note that despite the suggestions matching certain approaches for `symlink_auto`, the choices are still independent; there are ten viable combinations presented.

**Concerning `remove_*`**: I guess what’s done with the other three functions will guide what’s done with these three.

## Unsafe code in this library

On Windows only there is some unavoidable unsafe code in `remove_symlink_auto` to determine whether a symlink is a file symlink or a directory symlink, because this detail is not exposed in a stable function in the standard library.

## Author

[Chris Morgan](https://chrismorgan.info/) is the author and maintainer of this library.

## License

Copyright © 2017–2022 Chris Morgan

This project is distributed under the terms of three different licenses,
at your choice:

- Blue Oak Model License 1.0.0: https://blueoakcouncil.org/license/1.0.0
- MIT License: https://opensource.org/licenses/MIT
- Apache License, Version 2.0: https://www.apache.org/licenses/LICENSE-2.0

If you do not have particular cause to select the MIT or the Apache-2.0
license, Chris Morgan recommends that you select BlueOak-1.0.0, which is
better and simpler than both MIT and Apache-2.0, which are only offered
due to their greater recognition and their conventional use in the Rust
ecosystem. (BlueOak-1.0.0 was only published in March 2019.)

When using this code, ensure you comply with the terms of at least one of
these licenses.
