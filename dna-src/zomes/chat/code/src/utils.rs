use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        entry::{AppEntryValue, Entry},
        link::LinkMatch,
    },
    holochain_json_api::{
        json::{default_to_json, JsonString},
    },
    holochain_persistence_api::{
        cas::content::{ Address, AddressableContent },
    },
};
use serde::Serialize;
use std::{convert::TryFrom, fmt::Debug, collections::HashSet};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLinksLoadResult<T> {
    pub entry: T,
    pub address: Address,
}

impl<T: Into<JsonString> + Debug + Serialize> From<GetLinksLoadResult<T>> for JsonString {
    fn from(u: GetLinksLoadResult<T>) -> JsonString {
        default_to_json(u)
    }
}

///
/// Helper function that perfoms a try_from for every entry
/// of a get_links_and_load for a given type. Any entries that either fail to
/// load or cannot be converted to the type will be dropped.
///
pub fn get_links_and_load_type<R: TryFrom<AppEntryValue>>(
    base: &Address,
    link_type: LinkMatch<&str>,
    tag: LinkMatch<&str>,
) -> ZomeApiResult<Vec<GetLinksLoadResult<R>>> {
    let link_load_results = hdk::get_links_and_load(base, link_type, tag)?;

    Ok(link_load_results
        .iter()
        .map(|maybe_entry| match maybe_entry {
            Ok(entry) => match entry {
                Entry::App(_, entry_value) => {
                    let typed_entry = R::try_from(entry_value.to_owned()).map_err(|_| {
                        ZomeApiError::Internal(
                            "Could not convert get_links result to requested type".to_string(),
                        )
                    })?;
                    Ok((entry.address(), typed_entry))
                }
                _ => Err(ZomeApiError::Internal(
                    "get_links did not return an app entry".to_string(),
                )),
            },
            _ => Err(ZomeApiError::Internal(
                "get_links did not return an app entry".to_string(),
            )),
        })
        .filter_map(Result::ok)
        .map(|(address, entry)| GetLinksLoadResult { address, entry })
        .collect())
}

pub trait DagList {
    fn author<E: Into<JsonString> + Clone>(
        &mut self,
        table: &str,
        content: E,
        prev_authored: Option<Address>,
        prev_foreign: Option<Address>,
    ) -> ZomeApiResult<Address>;

    fn author_root_address(&self) -> Address;

    fn most_recent_authored(&self, table: &str) -> ZomeApiResult<Option<Address>>;

    fn get_prev_authored(&self, address: &Address) -> ZomeApiResult<Option<Address>>;

    fn get_prev_foreign(&self, address: &Address) -> ZomeApiResult<Option<Address>>;

    fn get_next(&self, table: &str, address: &Address) -> ZomeApiResult<Vec<Address>>;

    fn add_content_dag<E: Into<JsonString> + Clone>(&mut self, table: &str, content: E, fallback_root: &Address) -> ZomeApiResult<Address> {
        // get the most recent address of entry this agent authored (or some starting point)
        let most_recent_authored = self.most_recent_authored(table)?
            .unwrap_or(self.author_root_address());
        // get the entries after this one all the way to the tip (or some starting point)
        let most_recent_foreign = self.get_content_dag(table, &most_recent_authored, None, None)?.0.last().cloned()
            .unwrap_or(fallback_root.clone());
        self.author(table, content, Some(most_recent_authored), Some(most_recent_foreign))
    }

    fn get_content_dag(&self, table: &str, since: &Address, limit: Option<usize>, _backsteps: Option<usize>) -> ZomeApiResult<(Vec<Address>, bool)> {
        // step back to find some suitable starting entries (skip for now and just use current)
        let current = since;

        // traverse the unknown graph and store the result
        // uses non-recursive DFS topological sort 
        // as described here https://sergebg.blogspot.com/2014/11/non-recursive-dfs-topological-sort.html
        let mut to_visit = vec![(current.clone(), false)];
        let mut visited = HashSet::<Address>::new();
        let mut sort_stack = vec![];
        let mut more = false;

        while let Some((current, postprocess)) = to_visit.pop() {
            if postprocess {
                sort_stack.push(current.clone());
            } else {
                // push a second time but with post_process=true
                to_visit.push((current.clone(), true));
                for next in self.get_next(table, &current)? {
                    if !visited.contains(&next) {
                        if limit.is_none() || visited.len() < limit.unwrap() {
                            to_visit.push((next.clone(), false));
                            visited.insert(next.clone());
                        } else {
                            more = true;
                        }
                    }
                }
            }
        }
        sort_stack.pop(); // don't include the 'since' hash
        sort_stack.reverse();
        Ok((sort_stack, more))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct DagItem {
    prev_authored: Address,
    prev_foreign: Address,
    pub content: JsonString,
    table: String,
}

use hdk::prelude::*;


pub fn dag_item_entry_def() -> ValidatingEntryType {
    entry!(
        name: "dag_entry_item", // add some randomness or something
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<DagItem>| {
            Ok(())
        },

        links: [
            to!(
                "dag_entry_item",
                link_type: "dag/next",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "%agent_id",
                link_type: "dag/author_root",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "public_conversation",
                link_type: "dag/foreign_root",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

pub struct DhtDagList {}

impl DagList for DhtDagList {
    fn author<E: Into<JsonString> + Clone>(
        &mut self,
        table: &str,
        content: E,
        prev_authored: Option<Address>,
        prev_foreign: Option<Address>,
    ) -> ZomeApiResult<Address> {
        let entry = Entry::App(
            "dag_entry_item".into(),
            DagItem {
                prev_authored: prev_authored.clone().unwrap(),
                prev_foreign: prev_foreign.clone().unwrap(),
                content: content.into(),
                table: String::from(table),
            }.into()
        );
        let entry_addr = hdk::commit_entry(&entry)?;
        if let Some(prev_authored) = prev_authored {
            hdk::link_entries(&prev_authored, &entry_addr, "dag/next", table).or_else(|_| {
                hdk::link_entries(&prev_authored, &entry_addr, "dag/author_root", table)
            })?;
        }
        if let Some(prev_foreign) = prev_foreign {
            hdk::link_entries(&prev_foreign, &entry_addr, "dag/next", table).or_else(|_| {
                hdk::link_entries(&prev_foreign, &entry_addr, "dag/foreign_root", table)
            })?;
        }
        Ok(entry_addr)
    }

    fn author_root_address(&self) -> Address {
        Address::from(hdk::AGENT_ADDRESS.to_string())
    }

    fn get_prev_authored(&self, address: &Address) ->  ZomeApiResult<Option<Address>> {
        if let Some(Entry::App(_, raw)) = hdk::get_entry(address)? {
            if let Ok(item) = DagItem::try_from(raw) {
                return Ok(Some(item.prev_authored))
            }
        }
        Ok(None)
    }

    fn get_prev_foreign(&self, address: &Address) -> ZomeApiResult<Option<Address>> {
        if let Some(Entry::App(_, raw)) = hdk::get_entry(address)? {
            if let Ok(item) = DagItem::try_from(raw) {
                return Ok(Some(item.prev_foreign))
            }
        }
        Ok(None)
    }

    fn most_recent_authored(&self, table: &str) -> ZomeApiResult<Option<Address>> {
        match hdk::query_result("dag_entry_item".into(), QueryArgsOptions{ entries: true, ..Default::default()})? {
            QueryResult::Entries(entries) => {
                Ok(entries.iter()
                .filter(|(_addr, entry)| {
                    match entry {
                        Entry::App(_, content) => {
                            let item = DagItem::try_from(content).unwrap();
                            item.table == table
                        }, 
                        _ => false
                    }
                })
                .map(|(addr, _entry)| addr.clone())
                .collect::<Vec<_>>().last().cloned())
            },
            _ => unreachable!()
        }
    }

    fn get_next(&self, table: &str, address: &Address) -> ZomeApiResult<Vec<Address>> {
        hdk::get_links(address, LinkMatch::Regex("dag/*"), LinkMatch::Exactly(table)).map(|results| {
            results.addresses()
        })
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;
    use super::*;

    // create a test mock graph store

    struct TestStore{
        entry_store: HashMap<Address, JsonString>, 
        forward_link_store: HashMap<Address, Vec<Address>>,
        prev_authored_link_store: HashMap<Address, Address>,
        prev_foreign_link_store: HashMap<Address, Address>,
        author_list: Vec<Address>,
    }

    impl TestStore {
        fn new() -> Self {
            Self {
                entry_store: HashMap::new(),
                forward_link_store: HashMap::new(),
                prev_authored_link_store: HashMap::new(),
                prev_foreign_link_store: HashMap::new(),
                author_list: Vec::new(),
            }
        }
    }

    impl DagList for TestStore {
        fn author<E: Into<JsonString> + Clone>(
            &mut self,
            _table: &str,
            content: E,
            prev_authored: Option<Address>,
            prev_foreign: Option<Address>,
        ) -> ZomeApiResult<Address> {
            let entry_address = Address::from(String::from(content.clone().into()));
            // add the new entry
            self.entry_store.insert(entry_address.clone(), content.into());
            self.forward_link_store.insert(entry_address.clone(), Vec::new());
            // add the links from and to previous entries
            if let Some(prev_authored) = prev_authored {
                if self.forward_link_store.get(&prev_authored).is_none() {
                    self.forward_link_store.insert(prev_authored.clone(), Vec::new());
                }
                self.forward_link_store.get_mut(&prev_authored).unwrap().push(entry_address.clone());
                self.prev_authored_link_store.insert(entry_address.clone(), prev_authored);
            }
            if let Some(prev_foreign) = prev_foreign {
                if self.forward_link_store.get(&prev_foreign).is_none() {
                    self.forward_link_store.insert(prev_foreign.clone(), Vec::new());
                }
                self.forward_link_store.get_mut(&prev_foreign).unwrap().push(entry_address.clone());
                self.prev_foreign_link_store.insert(entry_address.clone(), prev_foreign);
            }
            // add to the author list
            self.author_list.push(entry_address.clone());
            Ok(entry_address)
        }

        fn author_root_address(&self) -> Address {
            Address::from("agent_root")
        }

        fn get_prev_authored(&self, address: &Address) -> ZomeApiResult<Option<Address>> {
            Ok(self.prev_authored_link_store.get(address).cloned())
        }
    
        fn get_prev_foreign(&self, address: &Address) -> ZomeApiResult<Option<Address>> {
            Ok(self.prev_foreign_link_store.get(address).cloned())
        }

        fn most_recent_authored(&self, _table: &str) -> ZomeApiResult<Option<Address>> {
            Ok(self.author_list.last().cloned())
        }

        fn get_next(&self, _table: &str, address: &Address) -> ZomeApiResult<Vec<Address>> {
            Ok(self.forward_link_store.get(address).unwrap_or(&Vec::new()).to_vec())
        }
    }


    #[test]
    fn test_get_nothing() {
        let mut store = TestStore::new();
        let root_addr = store.author("test_table", -1, None, None).unwrap();
        // This retrieves everything
        assert_eq!(
            store.get_content_dag("test_table", &root_addr, None, None),
            Ok((vec![], false)),
        );
    }


    #[test]
    fn test_get_singleton() {
        // 0
        let mut store = TestStore::new();
        let addr = store.author("test_table", 0, None, None).unwrap();
        // This retrieves everything
        assert_eq!(
            store.get_content_dag("test_table", &addr, None, None),
            Ok((vec![], false)),
        );
    }

    #[test]
    fn test_get_2_chain() {
        // 0->1
        let mut store = TestStore::new();
        let root_addr = store.author("test_table", 0, None, None).unwrap();
        let tip_addr = store.author("test_table", 1, Some(root_addr.clone()), None).unwrap();

        // This retrieves everything if started at the root
        assert_eq!(
            store.get_content_dag("test_table", &root_addr, None, None),
            Ok((vec![tip_addr.clone()], false)),
        );
        // starting at the tip gets nothing
        assert_eq!(
            store.get_content_dag("test_table", &tip_addr, None, None),
            Ok((vec![], false)),
        );
    }

    #[test]
    fn test_get_fork() {
        // 0->1
        //  \>2
        let mut store = TestStore::new();
        let root_addr = store.author("test_table", 0, None, None).unwrap();
        let tip1_addr = store.author("test_table", 1, Some(root_addr.clone()), None).unwrap();
        let tip2_addr = store.author("test_table", 2, None, Some(root_addr.clone())).unwrap();

        // This retrieves everything if started at the root
        assert_eq!(
            store.get_content_dag("test_table", &root_addr, None, None),
            Ok((vec![tip1_addr.clone(), tip2_addr.clone()], false)),
        );
    }

    #[test]
    fn test_two_authors() {
        // 0->1->2->3
        //  \     \   \
        //   \     \   \
        //    \>10->11->12
        //    
        let mut store = TestStore::new();
        let addr0 = store.author("test_table", 0, None, None).unwrap();
        let addr1 = store.author("test_table", 1, Some(addr0.clone()), None).unwrap();
        let addr2 = store.author("test_table", 2, Some(addr1.clone()), None).unwrap();
        let addr3 = store.author("test_table", 3, Some(addr2.clone()), None).unwrap();

        let addr10 = store.author("test_table", 10, None, Some(addr0.clone())).unwrap();
        let addr11 = store.author("test_table", 11, Some(addr10.clone()), Some(addr2.clone())).unwrap();
        let addr12 = store.author("test_table", 12, Some(addr11.clone()), Some(addr3.clone())).unwrap();

        // This retrieves everything if started at the root
        assert_eq!(
            store.get_content_dag("test_table", &addr0, None, None).unwrap().0,
            vec![addr1.clone(), addr2.clone(), addr3.clone(), addr10.clone(), addr11.clone(), addr12.clone()],
        );

        // This retrieves only things after 2 if started at 2
        assert_eq!(
            store.get_content_dag("test_table", &addr2, None, None).unwrap().0,
            vec![addr3, addr11.clone(), addr12],
        );

        // The limit can be used to truncate
        assert_eq!(
            store.get_content_dag("test_table", &addr0, Some(3), None),
            Ok((vec![addr1.clone(), addr10.clone(), addr11.clone()], true)),
        );
    }

    #[test]
    fn test_add_single_content_dag() {
        let mut store = TestStore::new();
        let root_addr = store.author("test_table", -1, None, None).unwrap();
        let addr = store.add_content_dag("test_table", 0, &root_addr).unwrap();
        assert_eq!(
            store.get_content_dag("test_table", &root_addr, None, None),
            Ok((vec![addr], false)),
        );
    }

    #[test]
    fn test_add_chain_content_dag() {
        let mut store = TestStore::new();
        let root_addr = store.author("test_table", -1, None, None).unwrap();
        let addr0 = store.add_content_dag("test_table", 0, &root_addr).unwrap();
        let addr1 = store.add_content_dag("test_table", 1, &root_addr).unwrap();
        let addr2 = store.add_content_dag("test_table", 2, &root_addr).unwrap();
        let addr3 = store.add_content_dag("test_table", 3, &root_addr).unwrap();
        let addr4 = store.add_content_dag("test_table", 4, &root_addr).unwrap();
        let addr5 = store.add_content_dag("test_table", 5, &root_addr).unwrap();

        // can get the lot
        assert_eq!(
            store.get_content_dag("test_table", &root_addr, None, None),
            Ok((vec![addr0.clone(), addr1.clone(), addr2.clone(), addr3.clone(), addr4.clone(), addr5.clone()], false)),
        );

        // can limit starting at the root
        assert_eq!(
            store.get_content_dag("test_table", &root_addr, Some(3), None),
            Ok((vec![addr0.clone(), addr1.clone(), addr2.clone()], true)),
        );

        // can limit starting at a 'since'
        assert_eq!(
            store.get_content_dag("test_table", &addr1, Some(3), None),
            Ok((vec![addr2.clone(), addr3.clone(), addr4.clone()], true)),
        );
    }
}

