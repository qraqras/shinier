#!/bin/bash
for file in *.new; do
  mv "$file" "${file//.new/}"
done
