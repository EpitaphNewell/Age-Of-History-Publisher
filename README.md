<div align="center">
  <img src="https://images.cooltext.com/5704142.gif" width="551" height="95" alt="AOC2 Publisher" />
  <div>
  <div>
    <img src="https://img.shields.io/badge/Steam-%23000000.svg?logo=steam&logoColor=white" alt="Steam"/>
    <img src="https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white" alt="Rust"/>
    
  </div>
<hr>

<div align="left">
> [!NOTE]
> This programm requires Rust.
> https://www.rust-lang.org

  Setup Guide
  To use the provided code for creating and uploading a Steam workshop item, follow these steps to modify specific sections with your own data.



Locate the Following Code Section:
```rust
.start_item_update(steamworks::AppId(603850), published_id)
    .content_path(Path::new("D:/Projects/Emoji"))
    .preview_path(Path::new("D:/Projects/test.jpg"))
    .title("Item title")
    .description("Item description")
    .tags(Vec::<String>::new(), false)
    .visibility(steamworks::PublishedFileVisibility::Public)
```
Replace with Your Details:

Update the paths and details with your own content paths, preview image, title, and description:
```rust
.start_item_update(steamworks::AppId(603850), published_id)
    .content_path(Path::new("C:/Your/Content/Path"))
    .preview_path(Path::new("C:/Your/Preview/Image.jpg"))
    .title("Your Item Title")
    .description("A short description of your item")
    .tags(Vec::<String>::new(), false)
    .visibility(steamworks::PublishedFileVisibility::Public)
```
- Content Path: Replace "C:/Your/Content/Path" with the path to your content.
- Preview Path: Replace "C:/Your/Preview/Image.jpg" with the path to your preview image.
- Title: Replace "Your Item Title" with the title of your workshop item.
- Description: Replace "A short description of your item" with a description of your item.
- Run the Code:

Open your terminal (or run scpirt in vs code), navigate to your project directory, and execute the following commands:
```rust
cargo build
cargo run
```
Execution Flow:

The code will create a new workshop item and save the PublishedFileId to published_id.txt.
After a brief pause, it will read the PublishedFileId from the file and upload your content and preview image to Steam.
Ensure the file paths are correct and you have the necessary permissions to read/write the specified files.
</div>
