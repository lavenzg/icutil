use crate::zone_info::{Zone, ZoneInfo, ZoneRule};
use anyhow::{anyhow, Error, Result};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::{collections::HashMap, num::ParseIntError, path::Path, str::FromStr};

#[derive(Parser)]
#[grammar = "rb.pest"]
pub struct ResourceBundleParser;

const VERSION_KEY: &str = "TZVersion";
const ZONES_KEY: &str = "Zones";
const NAMES_KEY: &str = "Names";
const RULES_KEY: &str = "Rules";
const REGIONS_KEY: &str = "Regions";

const TRANS_KEY: &str = "trans";
const TYPE_OFFSETS_KEY: &str = "typeOffsets";
const TYPE_MAP_KEY: &str = "typeMap";
const TRANS_PRE32_KEY: &str = "transPre32";
const TRANS_POST32_KEY: &str = "transPost32";
const FINAL_RULE_KEY: &str = "finalRule";
const FINAL_YEAR_KEY: &str = "finalYear";
const FINAL_RAW_KEY: &str = "finalRaw";
const LINKS_KEY: &str = "links";

impl ResourceBundleParser {
    pub fn parse_to_zone_info<P: AsRef<Path>>(path: P) -> Result<ZoneInfo> {
        let input = std::fs::read_to_string(path)?;
        let mut rb = ResourceBundleParser::parse(Rule::resource_bundle, &input)?;
        let _id = rb.next().unwrap();
        let table = rb.next().unwrap();
        Self::parse_table(table.into_inner())
    }

    fn parse_table(mut table: Pairs<Rule>) -> Result<ZoneInfo> {
        let mut map = HashMap::new();
        while let Some(key) = table.next() {
            let resource = table.next().unwrap();
            map.insert(key.as_str().trim(), resource);
        }

        let zones = if let Some(zones) = map.remove(ZONES_KEY) {
            Self::parse_zones(zones)?
        } else {
            Default::default()
        };
        println!("Number of zones record: {}", zones.len());

        let names = if let Some(names) = map.remove(NAMES_KEY) {
            Self::parse_names(names)
        } else {
            Default::default()
        };
        println!("Number of names: {}", names.len());

        let regions = if let Some(regions) = map.remove(REGIONS_KEY) {
            Self::parse_regions(regions)
        } else {
            Default::default()
        };
        println!("Number of regions: {}", regions.len());

        let rules = if let Some(rules) = map.remove(RULES_KEY) {
            Self::parse_rules(rules)?
        } else {
            Default::default()
        };
        println!("Number of rules: {}", rules.len());

        Ok(ZoneInfo {
            version: "".to_string(),
            zones,
            names,
            rules,
            regions,
        })
    }

    fn parse_zones(zones: Pair<Rule>) -> Result<Vec<Zone>> {
        let mut elements = zones.into_inner();
        let mut zones = Vec::new();
        while let Some(element) = elements.next() {
            zones.push(Self::parse_zones_record(element)?);
        }

        Ok(zones)
    }

    fn parse_zones_record(zone: Pair<Rule>) -> Result<Zone> {
        match zone.as_rule() {
            Rule::integer => {
                let number = zone.into_inner().next().unwrap().as_str();
                let alias_to: u32 = number.parse()?;
                Ok(Zone::AliasTo(alias_to))
            }
            Rule::table => Self::parse_zone_details(zone.into_inner()),
            _ => unreachable!(),
        }
    }

    fn parse_zone_details(mut details: Pairs<Rule>) -> Result<Zone> {
        let mut resource_map = HashMap::new();
        while let Some(key) = details.next() {
            let resource = details.next().unwrap();
            resource_map.insert(key.as_str().trim(), resource);
        }
        let trans_pre32: Vec<i32> = if let Some(resource) = resource_map.remove(TRANS_PRE32_KEY) {
            Self::parse_intvector(resource)?
        } else {
            Default::default()
        };
        let trans: Vec<i32> = if let Some(resource) = resource_map.remove(TRANS_KEY) {
            Self::parse_intvector(resource)?
        } else {
            Default::default()
        };
        let trans_post32: Vec<i32> = if let Some(resource) = resource_map.remove(TRANS_POST32_KEY) {
            Self::parse_intvector(resource)?
        } else {
            Default::default()
        };
        let type_offsets: Vec<i64> = if let Some(resource) = resource_map.remove(TYPE_OFFSETS_KEY) {
            Self::parse_intvector(resource)?
        } else {
            Default::default()
        };
        let type_map: Vec<u8> = if let Some(resource) = resource_map.remove(TYPE_MAP_KEY) {
            Self::parse_bin(resource.into_inner().next().unwrap())?
        } else {
            Default::default()
        };
        let final_rule_id = resource_map
            .remove(FINAL_RULE_KEY)
            .map(|resource| Self::parse_string(resource.into_inner().next().unwrap()))
            .unwrap_or_default();
        let final_raw: i32 = if let Some(resource) = resource_map.remove(FINAL_RAW_KEY) {
            Self::parse_integer(resource.into_inner().next().unwrap())?
        } else {
            Default::default()
        };
        let final_year: i32 = if let Some(resource) = resource_map.remove(FINAL_YEAR_KEY) {
            Self::parse_integer(resource.into_inner().next().unwrap())?
        } else {
            Default::default()
        };
        let aliases: Vec<u32> = if let Some(resource) = resource_map.remove(LINKS_KEY) {
            Self::parse_intvector(resource)?
        } else {
            Default::default()
        };
        let zone_detail = Zone::Detail {
            trans_pre32,
            trans,
            trans_post32,
            type_offsets,
            type_map,
            final_rule_id,
            final_raw,
            final_year,
            aliases,
        };

        Ok(zone_detail)
    }

    fn parse_intvector<T: FromStr<Err = ParseIntError>>(vec: Pair<Rule>) -> Result<Vec<T>> {
        let mut ret: Vec<T> = Vec::new();
        let mut numbers = vec.into_inner();
        while let Some(number) = numbers.next() {
            ret.push(number.as_str().parse()?);
        }
        Ok(ret)
    }

    fn parse_string(s: Pair<Rule>) -> String {
        s.as_str().trim_matches('"').to_string()
    }

    fn parse_integer<T: FromStr<Err = ParseIntError>>(i: Pair<Rule>) -> Result<T> {
        i.as_str()
            .parse()
            .map_err(|err| anyhow!("Parse {} failed: {}", i.as_str(), err))
    }

    fn parse_bin(bin: Pair<Rule>) -> Result<Vec<u8>> {
        let s = bin.as_str().trim_matches('"');
        assert_eq!(s.len() % 2, 0);
        let mut ret = Vec::with_capacity(s.len() / 2);
        for hex_code in s.chars().array_chunks::<2>() {
            ret.push(u8::from_str_radix(&String::from_iter(hex_code), 16)?);
        }

        Ok(ret)
    }

    fn parse_names(names: Pair<Rule>) -> Vec<String> {
        Self::parse_string_array(names)
    }

    fn parse_regions(regions: Pair<Rule>) -> Vec<String> {
        Self::parse_string_array(regions)
    }

    fn parse_string_array(rule: Pair<Rule>) -> Vec<String> {
        let inner = rule.into_inner().next().unwrap().into_inner();
        inner.map(|s| s.as_str().to_string()).collect()
    }

    fn parse_rules(zone_rules: Pair<Rule>) -> Result<Vec<ZoneRule>> {
        let mut ret = Vec::new();
        let mut inner = zone_rules.into_inner();
        while let Some(key) = inner.next() {
            let rule_name = key.as_str().to_string();
            let resource = inner.next().unwrap();
            let rule: Vec<i32> = Self::parse_intvector(resource)?;
            ret.push(ZoneRule {
                id: rule_name,
                values: rule,
            });
        }
        Ok(ret)
    }
}
