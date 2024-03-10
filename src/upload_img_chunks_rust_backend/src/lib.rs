use candid::{CandidType, Decode, Encode};
use ic_cdk::export_candid;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cell::RefCell;
mod address;

type Memory = VirtualMemory<DefaultMemoryImpl>;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Default)]
struct Message {
    sender: u32,
    receiver: u32,
    message: String,
}
#[derive(CandidType, Serialize, Deserialize)]
struct MessageList(Vec<Message>);

impl Storable for MessageList {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(Decode!(bytes.as_ref(), Vec<Message>).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for Message {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static FILE_STABLE_STATE: RefCell<StableBTreeMap<(u32,u32), Vec<u8>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
    static CHAT_STABLE_STATE: RefCell<StableBTreeMap<(u32,u32), MessageList, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
        )
    );
    static CHAT_ONE_STABLE_STATE: RefCell<StableBTreeMap<(u32,u32), Message, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
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

#[ic_cdk_macros::update]
fn send_one_message(from: u32, to: u32, message: Message) -> Option<Message> {
    CHAT_ONE_STABLE_STATE.with(|p| p.borrow_mut().insert((from, to), message))
}

#[ic_cdk_macros::query]
fn get_one_messages(from: u32, to: u32) -> Option<Message> {
    CHAT_ONE_STABLE_STATE.with(|p| {
        let state = p.borrow();
        let res = state.get(&(from, to));
        res
    })
}

#[ic_cdk_macros::update]
fn send_message(from: u32, to: u32, message: MessageList) -> Option<MessageList> {
    CHAT_STABLE_STATE.with(|p| p.borrow_mut().insert((from, to), message))
}
#[ic_cdk_macros::query]
fn get_messages(from: u32, to: u32) -> Option<MessageList> {
    CHAT_STABLE_STATE.with(|p| {
        let state = p.borrow();

        let res = state.get(&(from, to));
        res
    })
}

#[ic_cdk_macros::query]
fn get_messages_page(from: u32, to: u32, page_number: usize) -> Option<Vec<Message>> {
    CHAT_STABLE_STATE.with(|p| {
        let state = p.borrow();

        if let Some(message_list) = state.get(&(from, to)) {
            let messages = &message_list.0;

            let page_size = 3;
            let total_messages = messages.len();

            // Calculate start and end indices for pagination
            let start_index = if total_messages > page_size * (page_number + 1) {
                total_messages - page_size * (page_number + 1)
            } else {
                0
            };
            let end_index = total_messages - page_size * page_number;

            // Reverse the slice before converting to Vec
            let sliced_messages = messages[start_index..end_index].iter().rev().cloned().collect();

            Some(sliced_messages)
        } else {
            None
        }
    })
}

export_candid!();
