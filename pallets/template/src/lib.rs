#![cfg_attr(not(feature = "std"), no_std)]



mod example;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use example::set::*;
// pub use example::ringbuffer::*;


// todo https://substrate.dev/docs/en/knowledgebase/runtime/storage
// todo https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
// todo https://substrate.dev/docs/en/knowledgebase/runtime/events
