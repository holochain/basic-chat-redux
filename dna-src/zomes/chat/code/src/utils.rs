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

pub trait DagStore {
    fn author<E: Into<JsonString> + Clone>(
        &mut self,
        content: E,
        prev_authored: Option<Address>,
        prev_foreign: Option<Address>,
    ) -> ZomeApiResult<Address>;

    fn most_recent_authored(&self) -> Option<Address>;

    // fn get_prev_authored(address: &Address) -> Option<Address>;
    // fn get_prev_foreign(address: &Address) -> Option<Address>;
    fn get_next(&self, address: &Address) -> Vec<Address>;

    fn add_content_dag<E: Into<JsonString> + Clone>(&mut self, content: E) -> ZomeApiResult<Address> {
        // get the most recent address of entry this agent authored (or some starting point)
        let most_recent_authored = self.most_recent_authored();
        // get the entries after this one all the way to the tip (or some starting point)
        let most_recent_foreign = self.get_content_dag(most_recent_authored.clone(), None, None)?.0.last().cloned();
        self.author(content, most_recent_authored, most_recent_foreign)
    }

    fn get_content_dag(&self, since: Option<Address>, limit: Option<usize>, _backsteps: Option<usize>) -> ZomeApiResult<(Vec<Address>, bool)> {
        // step back to find some suitable starting entries (skip for now and just use current)
        let current = since.unwrap();

        // traverse the unknown graph and store the result
        // uses non-recursive DFS topological sort 
        // as described here https://sergebg.blogspot.com/2014/11/non-recursive-dfs-topological-sort.html
        let mut to_visit = vec![(current, false)];
        let mut visited = HashSet::<Address>::new();
        let mut sort_stack = vec![];
        let mut more = false;

        while let Some((current, postprocess)) = to_visit.pop() {
            if postprocess {
                sort_stack.push(current.clone());
            } else {
                visited.insert(current.clone());
                to_visit.push((current.clone(), true));
                // this is for the limit feature. Need to account for nodes we will post process
                // as well as those done already + the current node
                let count_so_far = sort_stack.len() + to_visit.iter().filter(|e|e.1).count() + 1;

                for next in self.get_next(&current) {
                    if !visited.contains(&next) {
                        if !(limit.is_some() && count_so_far >= limit.unwrap()) {
                            to_visit.push((next, false));
                        } else {
                            more = true;
                        }
                    }
                }
            }
        }
        sort_stack.reverse();
        Ok((sort_stack, more))
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
    }

    impl TestStore {
        fn new() -> Self {
            Self{
                entry_store: HashMap::new(),
                forward_link_store: HashMap::new(),
            }
        }
    }

    impl DagStore for TestStore {
        fn author<E: Into<JsonString> + Clone>(
            &mut self,
            content: E,
            prev_authored: Option<Address>,
            prev_foreign: Option<Address>,
        ) -> ZomeApiResult<Address> {
            let entry_address = Address::from(String::from(content.clone().into()));
            // add the new entry
            self.entry_store.insert(entry_address.clone(), content.into());
            self.forward_link_store.insert(entry_address.clone(), Vec::new());
            // add the links from previous entries
            if let Some(prev_authored) = prev_authored {
                self.forward_link_store.get_mut(&prev_authored).unwrap().push(entry_address.clone());
            }
            if let Some(prev_foreign) = prev_foreign {
                self.forward_link_store.get_mut(&prev_foreign).unwrap().push(entry_address.clone());
            }
            Ok(entry_address)
        }

        fn most_recent_authored(&self) -> Option<Address> {
            None
        }

        fn get_next(&self, address: &Address) -> Vec<Address> {
            self.forward_link_store.get(address).unwrap_or(&Vec::new()).to_vec()
        }
    }

    #[test]
    fn test_get_singleton() {
        // 0
        let mut store = TestStore::new();
        let addr = store.author(0, None, None).unwrap();
        // This retrieves everything
        assert_eq!(
            store.get_content_dag(Some(addr.clone()), None, None),
            Ok((vec![addr], false)),
        );
    }

    #[test]
    fn test_get_2_chain() {
        // 0->1
        let mut store = TestStore::new();
        let root_addr = store.author(0, None, None).unwrap();
        let tip_addr = store.author(1, Some(root_addr.clone()), None).unwrap();

        // This retrieves everything if started at the root
        assert_eq!(
            store.get_content_dag(Some(root_addr.clone()), None, None),
            Ok((vec![root_addr, tip_addr.clone()], false)),
        );
        // this retrieves only the tip if startred there
        assert_eq!(
            store.get_content_dag(Some(tip_addr.clone()), None, None),
            Ok((vec![tip_addr], false)),
        );
    }

    #[test]
    fn test_get_fork() {
        // 0->1
        //  \>2
        let mut store = TestStore::new();
        let root_addr = store.author(0, None, None).unwrap();
        let tip1_addr = store.author(1, Some(root_addr.clone()), None).unwrap();
        let tip2_addr = store.author(2, None, Some(root_addr.clone())).unwrap();

        // This retrieves everything if started at the root
        assert_eq!(
            store.get_content_dag(Some(root_addr.clone()), None, None),
            Ok((vec![root_addr, tip1_addr.clone(), tip2_addr.clone()], false)),
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
        let addr0 = store.author(0, None, None).unwrap();
        let addr1 = store.author(1, Some(addr0.clone()), None).unwrap();
        let addr2 = store.author(2, Some(addr1.clone()), None).unwrap();
        let addr3 = store.author(3, Some(addr2.clone()), None).unwrap();

        let addr10 = store.author(10, None, Some(addr0.clone())).unwrap();
        let addr11 = store.author(11, Some(addr10.clone()), Some(addr2.clone())).unwrap();
        let addr12 = store.author(12, Some(addr11.clone()), Some(addr3.clone())).unwrap();

        // This retrieves everything if started at the root
        assert_eq!(
            store.get_content_dag(Some(addr0.clone()), None, None).unwrap().0,
            vec![addr0.clone(), addr1.clone(), addr2.clone(), addr3.clone(), addr10.clone(), addr11.clone(), addr12.clone()],
        );

        // This retrieves only things after 2 if started at 2
        assert_eq!(
            store.get_content_dag(Some(addr2.clone()), None, None).unwrap().0,
            vec![addr2, addr3, addr11, addr12],
        );

        // The limit can be used to truncate
        assert_eq!(
            store.get_content_dag(Some(addr0.clone()), Some(3), None),
            Ok((vec![addr0, addr1.clone(), addr10.clone()], true)),
        );
    }
}

