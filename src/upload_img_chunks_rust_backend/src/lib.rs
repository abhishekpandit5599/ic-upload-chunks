use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static FILE_STABLE_STATE: RefCell<StableBTreeMap<(u32,u32), Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}

// Retrieves the value associated with the given key if it exists.
#[ic_cdk_macros::query]
fn get_image(key1: u32, key2: u32) -> Option<Vec<u8>> {
    FILE_STABLE_STATE.with(|p| p.borrow().get(&(key1, key2)))
}

// Inserts an entry into the map and returns the previous value of the key if it exists.
#[ic_cdk_macros::update]
fn upload_image(key1: u32, key2: u32, value: Vec<u8>) -> Option<Vec<u8>> {
    FILE_STABLE_STATE.with(|p| p.borrow_mut().insert((key1, key2), value))
}