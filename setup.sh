#!/bin/bash

source_dir=swish_raw
output_dir=swish_ocr

filecount=$(ls $source_dir | wc -l)

for i in $source_dir/*.jpg;
do 
    filename=$(basename $i)
    tesseract $source_dir/$filename $output_dir/$filename -l swe --psm 4
    echo "Completed $filename"
done

# grep -E "[0-9]+ kr$" swish_ocr/*.txt > rawdata.txt
grep -E "[^:-] [0-9]+ kr$" swish_ocr/*.txt > rawdata.txt