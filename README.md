# `coopted_llvm_sys`

This crate links against rustc‘s own LLVM and thus there may be dragons laying
in unsuspected places. Please do not use it, and if you must then do so only
from build scripts, as arbitrary programs are not guaranteed to run next to
the rustc that compiled them.

No higher-level bindings are provided by this crate, I'll probably do
`coopted_llvm` on top of it at a later date. Note too that even this `sys` crate
only provides the bare minimum to compile LLVM IR modules to native code.

Don't ask me why I did this, I just felt like it.

## Licensing

This crate is licensed under both the Apache 2.0 and MIT licenses, so you are
free to do whatever you want with it as long as you respect the terms from
these two.

If you are a highly paid worker at Google, Facebook, Apple, Amazon, Microsoft,
Palantir, Uber, Airbnb, Deliveroo, or any other company that prioritises profit
over people as strongly as they do, you can still use this crate. I simply wish
you will unionise and push back against the obsession for growth, control, and
power that is rampant in your workplace. Please take a stand against the
horrible working conditions they inflict on your lesser paid colleagues, and
more generally their gross disrespect for the very human rights they claim to
fight for.
