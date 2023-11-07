use crate::zone_info::ZoneInfo;
use pest::{Parser, iterators::{Pair, Pairs}};
use std::{path::Path, collections::HashMap, str::FromStr, num::ParseIntError};
use pest_derive::Parser;
use anyhow::{Result, anyhow, Error};

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
        Self::parse_table(table.into_inner());

        Ok(ZoneInfo {
            version: "".to_string(),
            zones: Vec::new(),
            names: Vec::new(),
            rules: Vec::new(),
            regions: Vec::new(),
        })
    }

    fn parse_table(mut table: Pairs<Rule>) {
        let mut map = HashMap::new();
        while let Some(key) = table.next() {
            let resource = table.next().unwrap();
            map.insert(key.as_str().trim(), resource);
        }
        
        if let Some(zones) = map.remove(ZONES_KEY) {
            Self::parse_zones(zones);
        }

        if let Some(names) = map.remove(NAMES_KEY) {
            Self::parse_names(names);
        }
    }

    fn parse_zones(zones: Pair<Rule>) {
        let mut elements = zones.into_inner();
        while let Some(element) = elements.next() {
            Self::parse_zones_record(element);
        }
    }
    
    fn parse_zones_record(zone: Pair<Rule>) -> Result<()> {
        match zone.as_rule() {
            Rule::integer => {
                let number = zone.into_inner().next().unwrap().as_str();
                let alias_to: u32 = number.parse()?;
            }
            Rule::table => {
                Self::parse_zone_details(zone.into_inner());
            }
            _ => unreachable!()
        }

        Ok(())
    }

    fn parse_zone_details(mut details: Pairs<Rule>) -> Result<()> {
        while let Some(key) = details.next() {
            let resource = details.next().unwrap();
            match key.as_str().trim() {
                TRANS_PRE32_KEY => {
                    let trans_pre32_vec: Vec<i32> = Self::parse_intvector(resource)?; 
                }
                TRANS_KEY => {
                    let trans_vec: Vec<i32> = Self::parse_intvector(resource)?; 
                }
                TRANS_POST32_KEY => {
                    let trans_post_vec: Vec<i32> = Self::parse_intvector(resource)?; 
                }
                TYPE_OFFSETS_KEY => {
                    let type_offsets_vec: Vec<i64> = Self::parse_intvector(resource)?; 
                }
                TYPE_MAP_KEY => {
                    let type_map: Vec<u8> = Self::parse_intvector(resource)?; 
                }
                FINAL_RULE_KEY => {
                    let final_rule = Self::parse_string(resource);
                }
                FINAL_RAW_KEY => {
                    let final_raw: u32 = Self::parse_integer(resource)?; 
                }
                FINAL_YEAR_KEY => {
                    let final_year: u32 = Self::parse_integer(resource)?; 
                }
                LINKS_KEY => {
                    let aliases: Vec<u32> = Self::parse_intvector(resource)?;
                }
                _ => unreachable!()
            }
        }

        Ok(())
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
        s.as_str().to_string()
    }
    
    fn parse_integer<T: FromStr<Err = ParseIntError>>(i: Pair<Rule>) -> Result<T> {
        i.as_str().parse().map_err(Error::msg)
    }
    
    fn parse_names(names: Pair<Rule>) {
        
    }
    
    fn parse_rules(rules: Pair<Rule>) {
        
    }
    
    fn parse_regions(regions: Pair<Rule>) {

    }
}
