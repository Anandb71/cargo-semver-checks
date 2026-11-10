# Type Alias Missing Lint - Implementation Status

## Issue
GitHub Issue: https://github.com/obi1kenobi/cargo-semver-checks/issues/1786

## Problem
Renaming `pub type Foo = Something` to `pub type Bar = Something` is not detected as a semver break.

## Root Cause
The `trustfall-rustdoc-adapter` GraphQL schema does not currently expose `TypeAlias` as an item type, even though `TypeAlias` exists in `rustdoc-types` (see https://docs.rs/rustdoc-types/latest/rustdoc_types/enum.ItemEnum.html).

## Current Implementation Status
This branch contains a complete implementation of the `type_alias_missing` lint that is ready to work once `TypeAlias` support is added to the adapter schema:

1. **Lint Definition**: `src/lints/type_alias_missing.ron` - follows the same pattern as `function_missing` and `struct_missing`
2. **Test Crates**: `test_crates/type_alias_missing/old` and `test_crates/type_alias_missing/new` - demonstrate the breaking change
3. **Test Registration**: Lint is registered in `src/query.rs`
4. **Test Output**: Placeholder snapshot file at `test_outputs/query_execution/type_alias_missing.snap`

## Test Results
Running `cargo test --lib tests_lints::type_alias_missing` produces:
```
ValidationError(NonExistentType("TypeAlias"))
```

This confirms that `TypeAlias` is not available in the current schema.

## Related Issues
- #1254: `pub type` type aliases have unclear public API participation
- #473: Type aliases of types defined outside the current crate are not checked
- #609: False positive when replacing a struct with a `pub type` alias
- #895: `#[doc(hidden)] pub type` with non hidden aliases

## Next Steps
1. Add `TypeAlias` support to `trustfall-rustdoc-adapter` schema
2. Update `cargo-semver-checks` to use the new adapter version
3. Run tests to generate the expected snapshot output
4. Ensure the lint correctly:
   - Detects removed public type aliases
   - Detects renamed public type aliases
   - Ignores private type aliases
   - Ignores type aliases in private modules
   - Ignores `#[doc(hidden)]` type aliases

## Technical Details
The lint query follows the established pattern:
- Query `baseline` for public type aliases with importable paths
- Assert their absence in `current`
- Filter by `visibility_limit = "public"` and `public_api = true`
- Output span information for error reporting
