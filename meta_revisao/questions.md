# Binary Tree Vertical Order Traversal
## Return vertical order traversal of its nodes

Do a BFS in the tree, so we have left -> right ordering
Go adding the nodes into a BTreeMap, with the column as key and the vec of node values in col as value


# Valid Word Abbreviation
substition -> s10n

For every char in the word
- if its a number, if it is a '0' and the number has not begun, return false
    - else, go building the number
- if its a letter, advance the w_pos by the number (0 if previous char was letter) and set number to 0
    - also if w_pos is out of bounds or if curr char is not equal to the w_pos found, return false
    - do w_pos += 1
- if its not a number or letter, return false

if w_pos + number == word.len(), return true
    else false


# Minimum remove to make valid parentheses
Since removing chars from strings is O(n), because all chars after removed char have to be shifted
We instead build a new string by parsing the old one

for every char in the old_string
- if its a close bracket, we remove an open from the stack, if there is one, else we skip this iteration and dont add it to string
- if its an open bracket, we add what will be its current position (len count) in the parsed string to the stack
we add the char to the parsed string and increment the len count

now, for every open that remained stacked, we remove it from the string :/
we can use replace_range for that, since its O(1) and substitute it with ""


# Nested list weight sum
remove the sum of every element times its weight depending on the list depth
we are given an enum of either Int(i32) or List(Vec<NestedInteger>), which makes it recursive

we just create a recursive function, that calls itself (with layer + 1) if its a list adding the result to sum
and if its an int, adds the int value * layer to sum


# LCA
## Do eulerian tour
then find min with a sparse table

## Or do two BFS


# Valid Palindrome
first transform the string to bytes, since we know its english letters only

now with a two pointer approach
if beginning != end, try to advance left or retreat right
and do a basic palindrome check with two pointers in that subslice


# Merge sorted array
simple merge function from merge sort
while first array element is bigger, advance first array
if second array element is bigger, insert it at that position (which shifts the other elements)
    pop an element from first array (which would be a 0 filling the array)
    advance second array

while n of advances is smaller than sizes of arrays and n of second array advances is smaller than size of second array
    first array receives the second array elements


# Basic Calculator
## Do with infix to postfix and then calculate

## Or do directly with makeshift solution by adding a + to the end, then calculating with a temp number
