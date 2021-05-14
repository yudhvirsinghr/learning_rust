#! /bin/sh

echo "Name of script:"
read script

#echo "Path to the code dir"
#read path
#
#echo "Script name"
#read script
#
rustc "$(path)/code/$script"
#-o "$(path)/binaries/$script"
