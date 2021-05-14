#! /bin/sh

echo "Path to the code:"
read path

echo "$path"

filename="$path" | cut -d '/' -f -1

echo $filename
#rustc $path -o "./binaries/"
