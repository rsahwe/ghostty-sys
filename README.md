# ghostty-sys

Bindgen bindings for ghostty.

The build uses zig as well a dynamic dependency that has to be included with the final executable. When libghostty is stable package managers might take over.

Currently the GHOSTTY_LOCATION env var has to be set to a folder containing libghostty.so (or equivalent).
