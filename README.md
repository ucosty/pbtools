# pbtools

![Build Status](https://github.com/ucosty/pbtools/workflows/build/badge.svg)


MacOS style clipboard utilities for the command line on X11 systems

## pbcopy

pbcopy takes the standard input and puts it into the clipboard

~~~shell script
# pbcopy < input_file

# echo "example input" | pbcopy
~~~

## pbpaste

pbpaste outputs the contents of the clipboard if it is a string

~~~shell script
# pbpaste > output_file
~~~

# Building

The pbtools are written in Rust, building the tools can be done with cargo

~~~shell script
# cargo build
~~~
