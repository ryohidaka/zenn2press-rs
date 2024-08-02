#!/bin/bash

cargo build --release


# Help command
target/release/zenn2press --help

printf '\n%s\n' ====================

# No params
target/release/zenn2press

printf '\n%s\n' ====================

# Set params
press_articles_dir="demo/press/docs/articles"
press_images_dir="demo/press/docs/public/images"

find $press_articles_dir -mindepth 1 -not -name '.gitignore' -delete
find $press_images_dir -mindepth 1 -not -name '.gitignore' -delete

target/release/zenn2press \
    -c demo/zenn2press-config.json \
    -d $press_articles_dir \
    -m $press_images_dir \
    -s demo/zenn \
    -i sample-article-1 

