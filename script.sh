#!/bin/bash

CWD=`pwd`

function recur() {  
    for folder in `ls "$1" -F | grep "/$" | tr ' ' '&'`
    do  
		folder=${folder//'&'/" "}
        if [[ `echo "$folder" | grep "[0-9]\."` ]]   
        then  
    		echo "$folder" 
			if [[ `ls "$1""/""$folder" | grep "cpp"` ]]
			then
				echo "cpp"
			fi
        else 
			recur "$1""/""$folder"
        fi  
    done  
}

recur $CWD 
