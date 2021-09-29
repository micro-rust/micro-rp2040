#!/bin/bash

# Settings.
ROWS=100
COLS=72

# Resize to preferred size.
resize -s $ROWS $COLS
stty rows $ROWS
stty cols $COLS


# Get current cargo project name.
base=${PWD}

# Get current cargo project name.
dirname=${PWD##*/}

# Trim trailing slashes.
name="${dirname%"${dirname##*[!/]}"}"
name="${name##*/}"


# Change to target directory.
cd ./target
cd ./thumb*
cd ./release


# ARM objcopy.
arm-none-eabi-objdump -d "$name"

bash