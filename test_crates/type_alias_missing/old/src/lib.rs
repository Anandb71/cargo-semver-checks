#![no_std]

// This type alias should be reported by the lint when it's removed
pub type PublicTypeAlias = u32;

// This type alias should also be reported when renamed
pub type WillBeRenamed = i64;

// This should not be reported - it's not pub
type PrivateTypeAlias = u64;

// This should not be reported - it's in a private module
mod private {
    pub type NonReachableTypeAlias = u8;
}

// This should not be reported - it's doc(hidden)
#[doc(hidden)]
pub type DocHiddenTypeAlias = i32;
