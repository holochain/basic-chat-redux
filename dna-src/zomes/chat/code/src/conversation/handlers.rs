use hdk::{
    self,
    error::ZomeApiResult,
    holochain_core_types::{entry::Entry, link::LinkMatch},
    holochain_json_api::json::{JsonString, RawString},
    holochain_persistence_api::cas::content::{Address, AddressableContent},
    AGENT_ADDRESS,

};
use std::collections::HashSet;
use crate::{
    DirectMessage,
    NotificationSignalPayload,
    JoinChannelSignalPayload,
    MESSAGE_ENTRY,
    PUBLIC_STREAM_LINK_TYPE_TO,
};
use crate::conversation::Conversation;
use crate::message;
use crate::utils::{get_links_and_load_type, GetLinksLoadResult};




fn notify_conversation(conversation_address: Address, message: DirectMessage) -> ZomeApiResult<()> {
    handle_get_members(conversation_address.clone())?
        .iter()
        .for_each(|member_id| {
            hdk::debug(format!("Send a message to: {:?}", &member_id.to_string())).ok();
            hdk::send(
                member_id.clone(),
                JsonString::from(
                    message.clone()
                ).into(),
                1.into(),
            )
            .ok();
        });
    Ok(())
}

fn notify_conversation_message(conversation_address: Address, message: message::Message) -> ZomeApiResult<()> {
    let message = DirectMessage::ChannelMessageNotification(
        NotificationSignalPayload{
            conversation_address: conversation_address.clone(),
            message: message.clone(),
            message_address: Entry::App(
                MESSAGE_ENTRY.into(),
                message.clone().into()
            ).address()
        }
    );
    notify_conversation(conversation_address, message)
}

fn notify_conversation_join(conversation_address: Address) -> ZomeApiResult<()> {
    let message = DirectMessage::JoinChannelNotification(
        JoinChannelSignalPayload{
            conversation_address: conversation_address.clone(),
            agent_address: AGENT_ADDRESS.to_string().into(),
        }
    );
    notify_conversation(conversation_address, message)
}

pub fn handle_start_conversation(
    name: String,
    description: String,
    initial_members: Vec<Address>,
) -> ZomeApiResult<Address> {
    let conversation = Conversation { name, description };
    let entry = Entry::App("public_conversation".into(), conversation.into());

    // first see if this converstion already exists.
    // while the entry will de-dup, the links will not adding useless data to the DHT.
    // There is also the possibility this chat has already been created but we just don't see it.
    // In this case there is nothing we can do except add it again and de-dup
    // in the get_channels zome function
    let conversation_address = entry.address();
    if hdk::get_entry(&conversation_address)?.is_none() {
        hdk::commit_entry(&entry)?;
        let anchor_entry = Entry::App(
            "anchor".into(),
            RawString::from("public_conversations").into(),
        );
        let anchor_address = hdk::commit_entry(&anchor_entry)?;
        hdk::link_entries(
            &anchor_address,
            &conversation_address,
            "public_conversation",
            "",
        )?;
    }
    let existing_members = handle_get_members(conversation_address.clone())?;

    // add the new members (including the creator)
    let mut members_to_add = vec![Address::from(AGENT_ADDRESS.to_string())];
    members_to_add.extend(initial_members);
    for member in members_to_add {
        if !existing_members.contains(&member) {
            hdk::link_entries(
                &conversation_address,
                &AGENT_ADDRESS,
                PUBLIC_STREAM_LINK_TYPE_TO,
                "",
            )?;
        }
    }
    Ok(conversation_address)
}

pub fn handle_join_conversation(conversation_address: Address) -> ZomeApiResult<()> {
    let existing_members = handle_get_members(conversation_address.clone())?;
    if !existing_members.contains(&AGENT_ADDRESS) {
        hdk::link_entries(
            &conversation_address,
            &AGENT_ADDRESS,
            PUBLIC_STREAM_LINK_TYPE_TO,
            "",
        )?;
    }
    notify_conversation_join(conversation_address)?;
    Ok(())
}

pub fn handle_get_members(address: Address) -> ZomeApiResult<Vec<Address>> {
    let all_member_ids =
        hdk::get_links(&address, LinkMatch::Exactly("has_member"), LinkMatch::Any)?
            .addresses()
            .to_owned();
    Ok(all_member_ids)
}

pub fn handle_get_messages(
    address: Address,
) -> ZomeApiResult<Vec<GetLinksLoadResult<message::Message>>> {
    get_links_and_load_type(&address, LinkMatch::Exactly("message_in"), LinkMatch::Any)
}

pub fn handle_post_message(
    conversation_address: Address,
    message_spec: message::MessageSpec,
) -> ZomeApiResult<()> {
    let message = message::Message::from_spec(&message_spec, &AGENT_ADDRESS.to_string());
    let message_entry = Entry::App("message".into(), message.clone().into());
    let message_addr = hdk::commit_entry(&message_entry)?;
    hdk::link_entries(&conversation_address, &message_addr, "message_in", "")?;
    // send the message direct as a signal to every agent in the channel
    notify_conversation_message(conversation_address, message)?;
    Ok(())
}

pub fn handle_get_all_public_conversations() -> ZomeApiResult<Vec<GetLinksLoadResult<Conversation>>>
{
    let anchor_entry = Entry::App(
        "anchor".into(),
        RawString::from("public_conversations").into(),
    );
    let anchor_address = hdk::entry_address(&anchor_entry)?;
    let mut result = get_links_and_load_type(
        &anchor_address,
        LinkMatch::Exactly("public_conversation"),
        LinkMatch::Any,
    )?;
    // dedup any channels that managed to slip through the dedup on write check
    // perhaps because we couldn't see them at the time of creation
    let mut uniques = HashSet::new();
    result.retain(|e| uniques.insert(e.address.clone()));
    Ok(result)
}
