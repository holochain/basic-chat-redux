use crate::member::Profile;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{entry::Entry, link::LinkMatch},
    holochain_json_api::json::RawString,
    holochain_persistence_api::cas::content::Address,
    AGENT_ADDRESS,
    prelude::{QueryResult, QueryArgsOptions},
};
use crate::utils::{get_links_and_load_type, GetLinksLoadResult};
use std::convert::TryFrom;

pub fn handle_register(name: String, avatar_url: String) -> ZomeApiResult<Address> {
    let anchor_entry = Entry::App("anchor".into(), RawString::from("member_directory").into());

    let anchor_address = hdk::commit_entry(&anchor_entry)?;
    hdk::link_entries(&anchor_address, &AGENT_ADDRESS, "member_tag", "")?;

    let profile_entry = Entry::App(
        "chat_profile".into(),
        Profile {
            name,
            avatar_url,
            address: AGENT_ADDRESS.to_string().into(),
        }
        .into(),
    );
    let profile_addr = hdk::commit_entry(&profile_entry)?;
    hdk::link_entries(&AGENT_ADDRESS, &profile_addr, "profile", "")?;

    Ok(AGENT_ADDRESS.to_string().into())
}

pub fn handle_get_member_profile(agent_address: Address) -> ZomeApiResult<Profile> {
    get_links_and_load_type(
        &agent_address,
        LinkMatch::Exactly("profile"),
        LinkMatch::Any,
    )?
    .iter()
    .next()
    .ok_or(ZomeApiError::Internal(
        "Agent does not have a profile registered".into(),
    ))
    .map(|elem: &GetLinksLoadResult<Profile>| elem.entry.clone())
}

/// Get your own member profile from your local chain always to save network requests
pub fn handle_get_my_member_profile() -> ZomeApiResult<Profile> {
    if let QueryResult::Entries(results) = hdk::query_result(
        "chat_profile".into(),
        QueryArgsOptions{ entries: true, ..Default::default()}
    )? {
        if let Some((_, Entry::App(_, entry_value))) = results.last() {
            Ok(Profile::try_from(entry_value).expect("Invalid profile data encountered"))
        } else {
            Err(ZomeApiError::Internal(
                "Agent does not have a profile registered".into(),
            ))
        }
    } else {
        unreachable!()
    }
}
