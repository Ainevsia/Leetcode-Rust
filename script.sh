#!/bin/bash

CWD=`pwd`

function recur() {  
    for folder in `ls "$1" -F | grep "/$" | tr ' ' '&'`
    do  
		folder=${folder//'&'/" "}
        if [[ `echo "$folder" | grep "[0-9]\."` ]]   
        then  
			# this is a problem folder, not problem class folder 
    		echo -n "| ${folder//'/'/''} |" 

			if [[ `ls "$1""$folder" | grep "toml"` ]]
			then
				# this is a rust project
				echo -n "[Rust](${1//'/home/ainevsia/gh/Leetcode-Rust'/'.'}${folder//' '/"%20"}src/main.rs)|"
			else
				echo -n "|"
			fi

			if [[ `ls "$1""$folder" | grep "cpp"` ]]
			then
				# this project contains cpp Solution
				file=`ls "$1""$folder" | grep "cpp"`
				echo -n "[C++](${1//'/home/ainevsia/gh/Leetcode-Rust'/'.'}${folder//' '/"%20"}"$file")|"
			else
				echo -n "|"
			fi

			if [[ `ls "$1""$folder" | grep "md"` ]]
			then
				# this project contains README.md
				solution=`ls "$1""$folder" | grep "md"` 
				echo -n "[Solution](${1//'/home/ainevsia/gh/Leetcode-Rust'/'.'}${folder//' '/"%20"}"$solution")|"
			else
				echo -n "|"
			fi

			echo
        else 
			# this is problem class folder , recur
			recur "$1""$folder"
        fi  
    done  
}

recur $CWD'/' | sort -V
