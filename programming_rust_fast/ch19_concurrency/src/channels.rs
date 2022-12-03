use std::{fs, io, thread};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc;

fn start_file_reader_thread(documents: Vec<PathBuf>)
    -> (mpsc::Receiver<String>, thread::JoinHandle<io::Result<()>>) {
    // channel() 方法产生一对值：发送者和接收者
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move|| {
        for document in documents {
            // 读取文件中的内容作为 String 存储在内存，复制给 text
            let text = fs::read_to_string(document)?;

            /*
            使用 sender move text 的值到 channel
            这里是直接传递 text 的值，
            而不是拷贝 text 的内容，
            无论 text 的是 10 行文本或者数兆文本，
            sender 都是将三个机器字的内容拷贝给接收者（String 结构的大小）,
            相应的 receiver.recv() 调用也是拷贝三个机器字的内容,
            send 和 recv 方法都是返回一个 Result ，
            这两个方法只有在 channel 的另一端被丢弃的时候才会失败，
            如果没有接收者，发送者发送的内容将会永远停留在 channel 中，
            如果没有发送者，接收者将永远处于等待状态无法接收到任何内容，
            挂断管道的方式是将管道的一端 drop 掉，
            确保这个时候将 channel 的连接关闭，
            所以 rust 的 channel 比 Unix 的管道效率更高
             */
            if sender.send(text).is_err() {
                break
            }
        }
        Ok(())
    });

    (receiver, handle)
}

fn start_file_indexing_thread(texts: mpsc::Receiver<String>)
    -> (mpsc::Receiver<String>, thread::JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();

    let handler = thread::spawn(move || {
        for (doc_id, text) in texts.into_iter().enumerate() {
            println!("doc id => {}, text => \n {}", doc_id, text.to_lowercase());
            let index = format!("doc id => {}", doc_id);
            if sender.send(index).is_err() {
                break
            }
        }
    });

    (receiver, handler)
}

fn write_index_to_local_file(indices: mpsc::Receiver<String>, output_path: PathBuf) -> io::Result<()> {
    let mut file = File::create(output_path)?;

    let handler = thread::spawn(move || {
        for index in indices.into_iter() {
            write!(file, "{}\n", index).unwrap()
        }
    });

    // 这是最后一个接收者，所以不需要在启动一个 pipeline
    handler.join().unwrap();

    Ok(())
}

fn run_pipeline(documents: Vec<PathBuf>, output_file: PathBuf)
    -> io::Result<()> {
    // 启动 pipeline 中所有的步骤
    let (texts, h1) = start_file_reader_thread(documents);
    let (indices, h2) = start_file_indexing_thread(texts);
    let result = write_index_to_local_file(indices, output_file);

    // 等待线程结束，保存所有可能遇到的错误
    let r1 = h1.join().unwrap();
    h2.join().unwrap();

    // 如有的话，返回第一个遇到的错误
    r1?;
    result
}

#[test]
fn test_pipeline() -> io::Result<()> {
    let docs = vec![
        PathBuf::from("/Users/liuanglin/data/welllist.txt"),
        PathBuf::from("/Users/liuanglin/data/notice.txt"),
    ];

    run_pipeline(docs, PathBuf::from("/Users/liuanglin/data/pipe.dat"))
}
