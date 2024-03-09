use crate::FILE_STABLE_STATE;

// Retrieves the value associated with the given key if it exists.
#[ic_cdk_macros::query]
fn get_image1(key1: u32, key2: u32) -> Option<Vec<u8>> {
    FILE_STABLE_STATE.with(|p| p.borrow().get(&(key1, key2)))
}