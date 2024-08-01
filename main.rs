use std::{fs::File, io::Write, path::Path, sync::mpsc::TryRecvError};
use steamworks::{Client, ClientManager, PublishedFileId, UGC};

fn create_item(ugc: &UGC<ClientManager>) {
    ugc.create_item(
        steamworks::AppId(603850),
        steamworks::FileType::Community,
        |create_result| {
            match create_result {
                Ok((published_id, _needs_to_agree_to_terms)) => {
                    println!("Published item with id {:?}", published_id);

                    // Записываем ID элемента в файл
                    if let Err(e) = write_id_to_file(published_id) {
                        eprintln!("Failed to write PublishedFileId to file: {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error creating workshop item: {:?}", e);
                }
            }
        },
    );
}

fn write_id_to_file(published_id: PublishedFileId) -> std::io::Result<()> {
    let path = Path::new("published_id.txt");
    let mut file = File::create(path)?;
    write!(file, "{}", published_id.0)?;
    Ok(())
}

fn upload_item_content(ugc: &UGC<ClientManager>, published_id: PublishedFileId) {
    let _upload_handle = ugc
        .start_item_update(steamworks::AppId(603850), published_id)
        .content_path(Path::new("D:/Projects/Emoji"))
        .preview_path(Path::new("D:/Projects/test.jpg"))
        .title("Item title")
        .description("Item description")
        .tags(Vec::<String>::new(), false)
        .visibility(steamworks::PublishedFileVisibility::Public)
        .submit(Some("My changenotes"), |upload_result| {
            match upload_result {
                Ok((published_id, needs_to_agree_to_terms)) => {
                    if needs_to_agree_to_terms {
                        println!("You need to agree to the terms of use before you can upload any files");
                    } else {
                        println!("Uploaded item with id {:?}", published_id);
                    }
                }
                Err(e) => {
                    eprintln!("Error uploading item: {:?}", e);
                }
            }
        });
}

fn read_id_from_file() -> Option<PublishedFileId> {
    let path = Path::new("published_id.txt");
    match std::fs::read_to_string(path) {
        Ok(id_str) => id_str.trim().parse().ok().map(PublishedFileId),
        Err(e) => {
            eprintln!("Failed to read file: {:?}", e);
            None
        }
    }
}

fn main() {
    let (client, single) = Client::init().expect("Steam is not running or has not been detected");

    let (tx, rx) = std::sync::mpsc::channel();
    let callback_thread = std::thread::spawn(move || {
        loop {
            single.run_callbacks();
            std::thread::sleep(std::time::Duration::from_millis(100));

            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }
        }
    });

    let ugc = client.ugc();
    
    // Создаем новый элемент и записываем его ID в файл
    create_item(&ugc);

    // Ожидаем немного времени, чтобы элемент был создан и ID записан в файл
    std::thread::sleep(std::time::Duration::from_secs(10));

    // Читаем ID из файла и обновляем элемент
    if let Some(published_id) = read_id_from_file() {
        upload_item_content(&ugc, published_id);
    } else {
        eprintln!("Failed to read PublishedFileId from file.");
    }

    tx.send(()).expect("Failed to send message to callback thread");
    callback_thread.join().expect("Failed to join callback thread");
}
