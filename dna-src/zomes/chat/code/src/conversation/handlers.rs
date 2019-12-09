use hdk::{
    self,
    error::ZomeApiResult,
    holochain_core_types::{entry::Entry, link::{LinkMatch, link_data::LinkData, LinkActionKind}},
    holochain_json_api::json::{JsonString, RawString},
    holochain_persistence_api::cas::content::{Address},
    AGENT_ADDRESS,
    prelude::{QueryResult, QueryArgsOptions},

};
use std::collections::HashSet;
use std::convert::TryFrom;
use crate::{
    DirectMessage,
    NotificationSignalPayload,
    JoinChannelSignalPayload,
    PUBLIC_STREAM_LINK_TYPE_TO,
    signal_ui,
};
use crate::conversation::Conversation;
use crate::message;
use crate::utils::{get_links_and_load_type, GetLinksLoadResult};
use crate::utils;
use crate::utils::DagList;



fn notify_conversation(conversation_address: Address, message: DirectMessage) -> ZomeApiResult<()> {
    handle_get_members(conversation_address.clone())?
        .iter()
        .for_each(|member_id| {
            if member_id == &Address::from(AGENT_ADDRESS.to_string()) { // don't waste resources and just trigger a signal directly
                signal_ui(&message);
            } else {
                hdk::debug(format!("Send a message to: {:?}", &member_id.to_string())).ok();
                hdk::send(
                    member_id.clone(),
                    JsonString::from(
                        message.clone()
                    ).into(),
                    1.into(),
                )
                .ok();
            }

        });
    Ok(())
}

fn notify_conversation_message(conversation_address: Address, message: message::Message, message_address: Address) -> ZomeApiResult<()> {
    let message = DirectMessage::ChannelMessageNotification(
        NotificationSignalPayload{
            conversation_address: conversation_address.clone(),
            message: message.clone(),
            message_address,
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
) -> ZomeApiResult<Address> {
    let conversation = Conversation { name, description };
    let entry = Entry::App("public_conversation".into(), conversation.into());
    let conversation_address = hdk::commit_entry(&entry)?;
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
    handle_join_conversation(conversation_address.clone())?;
    Ok(conversation_address)
}

fn entry_is_link_between(entry: &Entry, base: &Address, target: &Address) -> bool {
    if let Entry::LinkAdd(LinkData{
        action_kind: LinkActionKind::ADD,
        link,
        ..
    }) = entry {
        if link.base() == base && link.target() == target {
            return true
        }
    }
    false
}

/// An agent is a member of a channel if the have created a link between it and themselves in their local chain
fn agent_is_member_of_channel(agent_addr: &Address, conversation_address: &Address) -> ZomeApiResult<bool> {
    if let QueryResult::Entries(results) = hdk::query_result(
        "%link_add".into(),
        QueryArgsOptions{ entries: true, ..Default::default()}
    )? {
        Ok(
            results.iter().any(|(_, entry)| entry_is_link_between(entry, conversation_address, agent_addr))
        )
    } else {
        unreachable!()
    }
}

pub fn handle_join_conversation(conversation_address: Address) -> ZomeApiResult<()> {
    if !agent_is_member_of_channel(&AGENT_ADDRESS, &conversation_address)? {
        hdk::debug("Joining channel!").ok();
        hdk::link_entries(
            &conversation_address,
            &AGENT_ADDRESS,
            PUBLIC_STREAM_LINK_TYPE_TO,
            "",
        )?;
        notify_conversation_join(conversation_address)?;
    } else {
        hdk::debug("Already a member of channel!")?;
    }
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
    conversation_address: Address,
    since: Option<Address>,
    limit: Option<usize>,
) -> ZomeApiResult<Vec<GetLinksLoadResult<message::Message>>> {
    let dl = utils::DhtDagList{};
    let since = since.unwrap_or(conversation_address.clone());
    dl.get_content_dag(&String::from(conversation_address.clone()), &since, limit, None).map(|(addrs, _more)| {
        addrs.iter().filter_map(|address| {
            let entry = hdk::get_entry(&address).unwrap().unwrap();
            match entry {
                Entry::App(_, entry) => {
                    if let Ok(dag_entry) = utils::DagItem::try_from(entry) {                        
                        Some(GetLinksLoadResult {
                            entry: message::Message::try_from(dag_entry.content).unwrap(),
                            address: address.clone(),
                        })
                    } else {
                        None
                    }
                },
                _ => None
            }
        }).collect()
    })

}

pub fn handle_post_message(
    conversation_address: Address,
    message_spec: message::MessageSpec,
) -> ZomeApiResult<()> {
    let message = message::Message::from_spec(&message_spec, &AGENT_ADDRESS.to_string());
    let mut dl = utils::DhtDagList{};
    let message_item_addr = dl.add_content_dag(&String::from(conversation_address.clone()), message.clone(), &conversation_address)?;
    // send the message direct as a signal to every agent in the channel
    notify_conversation_message(conversation_address, message, message_item_addr)?;
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
