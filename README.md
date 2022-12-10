# Simple Calculator in CLI using Rust

This Rust code is a simple program that performs basic mathematical operations on a list of numbers. The program takes input from the user in the form of a string, which is then split into a list of individual numbers. The first word in the user's input string is expected to be the name of the operation that should be performed on the list of numbers, such as "add", "minus", or "multiply".

The program then calls one of the three functions add(), minus(), or multiply(), depending on the operation specified by the user. Each of these functions takes a Vec<&str> as an argument, which is the list of numbers that was extracted from the user's input string.

The add() function iterates through the list of numbers and adds them together, printing the result to the console. The multiply() function does the same thing, but multiplies the numbers together instead of adding them. Finally, the minus() function subtracts each number in the list from the first number in the list, and prints the result to the console.
