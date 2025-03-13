#!/bin/bash
clear
# Declare an array with 6 elements:
array=(one two three four five six)
# Print the first element:
	# echo "${array[0]}" # => "one"
# Print all elements:
	# echo "${array[@]}" # => "one two three four five six"
# Print the number of elements:
	# echo "${#array[@]}" # => "6"
# Print the number of characters in third element
	# echo "${#array[2]}" # => "5"
# Print 2 elements starting from fourth:
	# echo "${array[@]:3:2}" # => "four five"
# Print all elements each of them on new line.
for item in "${array[@]}"; do
    echo "$item"
done
