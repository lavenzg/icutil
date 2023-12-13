use anyhow::Error;
use anyhow::Result;
use itertools::Itertools;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct ZoneInfo {
    pub version: String,
    pub zones: Vec<Zone>,
    pub names: Vec<String>,
    pub rules: Vec<ZoneRule>,
    pub regions: Vec<String>,
}

#[derive(Debug)]
pub enum Zone {
    AliasTo(u32),
    Detail {
        trans_pre32: Vec<i32>,
        trans: Vec<i32>,
        trans_post32: Vec<i32>,
        type_offsets: Vec<i64>,
        type_map: Vec<u8>,
        final_rule_id: String,
        final_raw: i32,
        final_year: i32,
        aliases: Vec<u32>,
    },
}

#[derive(Debug)]
pub struct ZoneRule {
    pub id: String,
    pub values: Vec<i32>,
}

impl ZoneInfo {
    pub fn write_to<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        assert!(path.as_ref().is_dir());
        self.write_zone_aliases(path.as_ref().join("aliases.def"))?;
        self.write_zone_details(path.as_ref().join("zones.def"))?;
        self.write_rules(path.as_ref().join("rules.def"))?;
        self.write_names(path.as_ref().join("names.def"))?;
        self.write_regions(path.as_ref().join("regions.def"))?;

        Ok(())
    }

    pub fn write_rules<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut buf = Vec::new();
        for rule in &self.rules {
            writeln!(
                buf,
                "{{\"{}\", {{{}}}}},",
                rule.id,
                rule.values.iter().map(|n| n.to_string()).join(",")
            )?;
        }

        std::fs::write(path, buf).map_err(Error::msg)
    }

    pub fn write_names<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut buf = Vec::new();
        for name in &self.names {
            writeln!(buf, "\"{name}\",")?;
        }

        std::fs::write(path, buf).map_err(Error::msg)
    }

    pub fn write_regions<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut buf = Vec::new();
        for region in &self.regions {
            writeln!(buf, "\"{region}\",")?;
        }

        std::fs::write(path, buf).map_err(Error::msg)
    }

    pub fn write_zone_aliases<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut buf = Vec::new();
        for (index, zone) in self.zones.iter().enumerate() {
            if let Zone::AliasTo(alias_to) = zone {
                writeln!(buf, "{{{index}, {alias_to}}},")?;
            }
        }

        std::fs::write(path, buf).map_err(Error::msg)
    }

    pub fn write_zone_details<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut buf = Vec::new();
        for (index, zone) in self.zones.iter().enumerate() {
            if let Zone::Detail {
                trans_pre32,
                trans,
                trans_post32,
                type_offsets,
                type_map,
                final_rule_id,
                final_raw,
                final_year,
                aliases,
            } = zone
            {
                writeln!(
                    buf,
                    "{{{}, {{{}}}, {{{}}}, {{{}}}, {{{}}}, {{{}}}, \"{}\", {}, {}, {{{}}}}},",
                    index,
                    trans_pre32.iter().map(|n| n.to_string()).join(","),
                    trans.iter().map(|n| n.to_string()).join(","),
                    trans_post32.iter().map(|n| n.to_string()).join(","),
                    type_offsets.iter().map(|n| n.to_string()).join(","),
                    type_map.iter().map(|n| n.to_string()).join(","),
                    final_rule_id,
                    final_raw,
                    final_year,
                    aliases.iter().map(|n| n.to_string()).join(","),
                )?;
            }
        }

        std::fs::write(path, buf)?;

        Ok(())
    }
}
