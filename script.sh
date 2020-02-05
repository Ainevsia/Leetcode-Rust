#!/bin/bash

ls | sort -V | sed 's/ /%20/g' | awk -F '%20' '{printf "|"; for (i=1;i<=NF;i++) printf $i" "; print "|[Rust](./" $0 "/src/main.rs)|"}' >> README.md
