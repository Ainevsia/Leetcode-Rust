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
				echo -n "[C++](${1//'/home/ainevsia/gh/Leetcode-Rust'/'.'}${folder//' '/"%20"})|"
			fi
        else 
			# this is problem class folder , recur
			recur "$1""$folder"
        fi  
    done  
}

recur $CWD'/'
