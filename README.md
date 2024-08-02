# zenn2press

`zenn2press` is a Rust library to convert Zenn contents to VitePress.

## Features

### `copy_markdown_files()`

Copy Markdown files from a source directory to a destination directory with options to include or exclude specific files.

### `copy_images()`

Copy image files from a source directory to a destination directory, also with options to include or exclude specific files.

## Installation

Add zenn2press to your `Cargo.toml`:

```toml
[dependencies]
zenn2press = "0.1.0"
```

## Usage

### CLI

Here is an example of how to use zenn2press with cli:

```sh
zenn2press --help

zenn2press \
    -s zenn \
    -c config.json \
    -d press/docs/articles \
    -m press/docs/public/images \
    -i sample-article-1
```

### Package

Here is an example of how to use zenn2press in your project:

```rust
use zenn2press::{copy_images, copy_markdown_files};

#[tokio::main]
async fn main() {
    let src_dir = "demo/zenn/articles";
    let dest_dir = "demo/press/docs/articles";
    let config_file = Some("demo/zenn2press-config.json");
    let include = Some(vec!["sample-article-1"]);
    let exclude = None;

    // Copy markdown files from srcDir to destDir using properties
    copy_markdown_files(
        src_dir,
        dest_dir,
        config_file,
        include,
        exclude
    )
    .unwrap_or_else(|e| {
        eprintln!("Error copying markdown files: {}", e);
    });

    // Copy images from srcDir to destDir using properties
    match copy_images(
        "demo/zenn/images",
        "demo/press/docs/public/images",
        include,
        exclude
    )
    .await
    {
        Ok(_) => println!("Images copied successfully."),
        Err(e) => eprintln!("Error copying images: {}", e),
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
