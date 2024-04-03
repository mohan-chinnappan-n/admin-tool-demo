#!/bin/bash

#------------------------------------------------------
# generates package.xml for the given csv file (stdin)
# author: mohan chinnappan
#------------------------------------------------------

# Function to generate package.xml content
generate_package_xml() {
    # Map MemberType.MemberName to Members
    declare -A members_mapping
    # Map MemberType to Name
    declare -A name_mapping

    # Read CSV data from stdin and populate mappings
    while IFS=, read -r MemberType MemberName; do
        members_mapping["$MemberType.$MemberName"]+="$MemberName "
        name_mapping["$MemberType"]+="$MemberName "
    done

    # Generate package.xml content
    printf '<?xml version="1.0" encoding="UTF-8"?>\n<Package xmlns="http://soap.sforce.com/2006/04/metadata">\n  <version>58.0</version>\n'

    for MemberType in "${!name_mapping[@]}"; do
        printf '  <types>\n'
        for MemberName in ${name_mapping["$MemberType"]}; do
            MemberValues="${members_mapping["$MemberType.$MemberName"]}"
            printf "    <members>${MemberValues}</members>\n"
        done
        printf "    <name>$MemberType</name>\n  </types>\n"
    done

    printf "</Package>\n"
}

# Example usage
# Read CSV data from stdin
generate_package_xml
