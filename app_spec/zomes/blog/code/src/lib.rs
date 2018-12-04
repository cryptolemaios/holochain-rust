#![feature(try_from)]

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate boolinator;
#[macro_use]
extern crate holochain_core_types_derive;

pub mod blog;
pub mod post;

use hdk::holochain_core_types::cas::content::Address;
use hdk::error::ZomeApiResult;
use hdk::holochain_core_types::json::JsonString;
use hdk::holochain_core_types::entry::Entry;
use hdk::holochain_wasm_utils::api_serialization::get_links::GetLinksResult;

define_zome! {
    entries: [
        post::definition()
    ]

    genesis: || {
        Ok(())
    }

    functions: {
        main (Public) {
            check_sum: {
                inputs: |num1: u32, num2: u32|,
                outputs: |post: ZomeApiResult<JsonString>|,
                handler: blog::handle_check_sum
            }

            post_address: {
                inputs: |content: String|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: blog::handle_post_address
            }

            create_post: {
                inputs: |content: String, in_reply_to: Option<Address>|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: blog::handle_create_post
            }

            posts_by_agent: {
                inputs: |agent: Address|,
                outputs: |post_hashes: ZomeApiResult<GetLinksResult>|,
                handler: blog::handle_posts_by_agent
            }

            get_post: {
                inputs: |post_address: Address|,
                outputs: |post: ZomeApiResult<Option<Entry>>|,
                handler: blog::handle_get_post
            }

            my_posts: {
                inputs: | |,
                outputs: |post_hashes: ZomeApiResult<GetLinksResult>|,
                handler: blog::handle_my_posts
            }

            my_posts_as_committed: {
                inputs: | |,
                outputs: |post_hashes: ZomeApiResult<Vec<Address>>|,
                handler: blog::handle_my_posts_as_commited
            }
        }
    }
}
