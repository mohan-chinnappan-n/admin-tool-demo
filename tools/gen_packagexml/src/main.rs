use csv::ReaderBuilder;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() {
    // Read CSV data from stdin
    let mut csv_data = String::new();
    io::stdin().read_to_string(&mut csv_data).unwrap();

    // Generate package.xml content
    let package_xml_content = generate_package_xml(&csv_data);

    // Write the package.xml content to stdout
    println!("{}", package_xml_content);
}

fn generate_package_xml(csv_data: &str) -> String {
    let mut members_mapping: HashMap<String, HashSet<String>> = HashMap::new();
    let mut name_mapping: HashMap<String, HashSet<String>> = HashMap::new();

    // Read CSV data and populate mappings
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(csv_data.as_bytes());
    for result in reader.records() {
        if let Ok(record) = result {
            let member_name = record.get(1).unwrap().to_string();
            let member_type = record.get(0).unwrap().to_string();

            members_mapping
                .entry(format!("{}.{}", member_type, member_name))
                .or_insert(HashSet::new())
                .insert(member_name.clone());

            name_mapping
                .entry(member_type.clone())
                .or_insert(HashSet::new())
                .insert(member_name.clone());
        }
    }

    // Generate package.xml content
    let mut package_xml_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Package xmlns="http://soap.sforce.com/2006/04/metadata">
  <version>58.0</version>
"#
    );

    for (member_type, member_names) in &name_mapping {
        package_xml_content.push_str("  <types>\n");
        for member_name in member_names {
            let member_values = members_mapping
                .get(&format!("{}.{}", member_type, member_name))
                .unwrap_or(&HashSet::new())
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join(", ");
            package_xml_content.push_str(&format!(
                "    <members>{}</members>\n",
                member_values
            ));
        }
        package_xml_content.push_str(&format!("    <name>{}</name>\n  </types>\n", member_type));
    }

    package_xml_content.push_str("</Package>");

    package_xml_content
}
