#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
#[macro_use]
extern crate holochain_json_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
extern crate validator;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};
use std::convert::TryInto;

use hdk_proc_macros::zome;
use utils::GetLinksLoadResult;

pub mod anchor;
pub mod conversation;
pub mod member;
pub mod message;
mod utils;

pub static MESSAGE_ENTRY: &str = "message";
pub static MESSAGE_LINK_TYPE_TO: &str = "message_in";
pub static PUBLIC_STREAM_ENTRY: &str = "public_conversation";
pub static PUBLIC_STREAM_LINK_TYPE_TO: &str = "has_member";

pub const CHANNEL_MESSAGE_SIGNAL_TYPE: &str = "new_convo_message";
pub const JOIN_CHANNEL_SIGNAL_TYPE: &str = "join_convo_message";


#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
#[serde(rename_all = "camelCase")]
struct NotificationSignalPayload {
    conversation_address: Address,
    message_address: Address,
    message: message::Message,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JoinChannelSignalPayload {
    conversation_address: Address,
    agent_address: Address,
}

/// Fully typed definition of the types of direct messages
#[derive(Clone, Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
enum DirectMessage {
	ChannelMessageNotification(NotificationSignalPayload),
	JoinChannelNotification(JoinChannelSignalPayload)
}


#[derive(Debug, Serialize, Deserialize, DefaultJson)]
#[serde(rename_all = "camelCase")]
struct NamePayload {
    name: String,
}

pub (crate) fn signal_ui(message: &DirectMessage) {
    match message {
        DirectMessage::ChannelMessageNotification(signal_payload) => {
            // send a signal to the UI which it can use to reactively display messages
            hdk::emit_signal(
                CHANNEL_MESSAGE_SIGNAL_TYPE,
                signal_payload,
            ).ok();
        },
        DirectMessage::JoinChannelNotification(signal_payload) => {
            // signal the UI that a new agent has joined
            hdk::emit_signal(
                JOIN_CHANNEL_SIGNAL_TYPE,
                signal_payload,
            ).ok();
        }
    };
}

#[zome]
pub mod chat {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[receive]
    pub fn receive(from: Address, msg_json: JsonString) -> String {
        hdk::debug(format!("New direct message from: {:?}", from)).ok();
        let maybe_message: Result<DirectMessage, _> = JsonString::from_json(&msg_json).try_into();
        match maybe_message {
            Err(err) => format!("Err({})", err),
            Ok(message) => {
            	signal_ui(&message);
	            String::from("Ok")
            },
        }
    }

    #[entry_def]
    pub fn dag_item_entry_def() -> ValidatingEntryType {
        utils::dag_item_entry_def()
    }

    #[entry_def]
    pub fn message_entry_def() -> ValidatingEntryType {
        message::message_definition()
    }

    #[entry_def]
    pub fn public_conversation_entry_def() -> ValidatingEntryType {
        conversation::public_conversation_definition()
    }

    #[entry_def]
    pub fn member_entry_def() -> ValidatingEntryType {
        member::profile_definition()
    }

    #[entry_def]
    pub fn anchor_entry_def() -> ValidatingEntryType {
        anchor::anchor_definition()
    }

    #[zome_fn("hc_public")]
    pub fn register(name: String, avatar_url: String) -> ZomeApiResult<Address> {
        member::handlers::handle_register(name, avatar_url)
    }

    #[zome_fn("hc_public")]
    pub fn start_conversation(
        name: String,
        description: String,
    ) -> ZomeApiResult<Address> {
        conversation::handlers::handle_start_conversation(name, description)
    }

    #[zome_fn("hc_public")]
    pub fn join_conversation(conversation_address: Address) -> ZomeApiResult<()> {
        conversation::handlers::handle_join_conversation(conversation_address)
    }

    #[zome_fn("hc_public")]
    pub fn get_all_public_conversations(
    ) -> ZomeApiResult<Vec<GetLinksLoadResult<conversation::Conversation>>> {
        conversation::handlers::handle_get_all_public_conversations()
    }

    #[zome_fn("hc_public")]
    pub fn get_members(conversation_address: Address) -> ZomeApiResult<Vec<Address>> {
        conversation::handlers::handle_get_members(conversation_address)
    }

    #[zome_fn("hc_public")]
    pub fn get_member_profile(agent_address: Address) -> ZomeApiResult<member::Profile> {
        member::handlers::handle_get_member_profile(agent_address)
    }

    #[zome_fn("hc_public")]
    pub fn get_my_member_profile() -> ZomeApiResult<member::Profile> {
        member::handlers::handle_get_my_member_profile()
    }

    #[zome_fn("hc_public")]
    pub fn post_message(
        conversation_address: Address,
        message: message::MessageSpec,
    ) -> ZomeApiResult<()> {
        conversation::handlers::handle_post_message(conversation_address, message)
    }

    #[zome_fn("hc_public")]
    pub fn get_messages(
        address: Address,
    ) -> ZomeApiResult<Vec<GetLinksLoadResult<message::Message>>> {
        conversation::handlers::handle_get_messages(address)
    }
}
