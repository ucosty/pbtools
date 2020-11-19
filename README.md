# pbtools

MacOS style clipboard utilities for the command line

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
