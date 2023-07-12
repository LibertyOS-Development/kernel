# Consolidate Macros - July 10, 2023 - Daniel Teberian

Could we have a single module that handles all our macros? We already have crate::macros, but it does not include all of the macros. Could we move all the macros into that module, so that we can reference "crate::macros::<macro>" in other modules?
