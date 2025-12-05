use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read};

use crate::{
    prefix_paths::PrefixPaths,
    property::{PropertySource, PropertyValue},
};

#[derive(Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct Property {
    pub value: PropertyValue,
    #[serde(skip)]
    pub source: Option<PropertySource>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ZfsSpecificationDataset {
    pub properties: HashMap<String, Property>,
}

impl ZfsSpecificationDataset {
    pub fn new<S>(properties: HashMap<S, PropertyValue>) -> ZfsSpecificationDataset
    where
        S: AsRef<str>,
    {
        ZfsSpecificationDataset {
            properties: properties
                .into_iter()
                .map(|(k, value)| {
                    (
                        k.as_ref().to_owned(),
                        Property {
                            value,
                            source: None,
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn get_property<S>(&self, name: S) -> Option<&Property>
    where
        S: AsRef<str>,
    {
        self.properties.get(name.as_ref())
    }
}

#[derive(Deserialize, Serialize)]
pub struct ZfsSpecification {
    pub datasets: HashMap<String, ZfsSpecificationDataset>,
}

impl ZfsSpecification {
    pub fn from_reader<R>(rdr: R) -> Result<ZfsSpecification, serde_json::Error>
    where
        R: Read,
    {
        let mut res: ZfsSpecification = serde_json::from_reader(rdr)?;
        res.expand_sub_datasets();
        Ok(res)
    }

    pub fn get_dataset<S>(&self, name: S) -> Option<&ZfsSpecificationDataset>
    where
        S: AsRef<str>,
    {
        self.datasets.get(name.as_ref())
    }

    pub fn expand_sub_datasets(&mut self) {
        let mut datasets_sorted = self
            .datasets
            .iter()
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>();
        datasets_sorted.sort_by_key(String::len);

        for name in datasets_sorted {
            let name = name.clone();
            for dataset_prefix in PrefixPaths::new(&name) {
                self.datasets.entry(dataset_prefix.to_string()).or_insert(
                    ZfsSpecificationDataset {
                        properties: HashMap::new(),
                    },
                );
            }
        }
    }
}
