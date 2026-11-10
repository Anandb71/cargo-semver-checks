#![no_std]

// PublicTypeAlias was removed - should be reported

// WillBeRenamed was renamed to NewName - should be reported
pub type NewName = i64;

// PrivateTypeAlias was removed - should not be reported (wasn't pub)

// NonReachableTypeAlias was removed - should not be reported (wasn't reachable)

// DocHiddenTypeAlias was removed - should not be reported (was doc(hidden))
