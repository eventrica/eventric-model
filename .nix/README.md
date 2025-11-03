# Nix

This directory contains a simple Nix Flake which can be used to provide an appropriate
Rust environment for development. This can be made automatic using `direnv`, by creating
a `.envrc` file in the root of the repository, containing the following (presuming
that your installation of `direnv` has Nix Flake support):

```
use flake .nix
```

For security reasons, this file is ignored by Git.