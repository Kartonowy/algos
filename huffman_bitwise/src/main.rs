use core::fmt;
use std::{
    borrow::BorrowMut,
    fs::{self, File},
    io::{self, Read, Write},
};

struct ByteWriter<W>
where
    W: Write,
{
    writer: W,
    capacity: u8,
    slots: u8,
}
#[derive(Debug, Clone, Copy)]
struct Package<T> {
    content: T,
    length: u8,
}
impl<T> Package<T> {
    fn new(content: T, length: u8) -> Package<T> {
        Package { content, length }
    }
}
impl<T> fmt::Display for Package<T>
where
    T: fmt::Binary,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(content {:b}, length {})", self.content, self.length)
    }
}

impl<W> ByteWriter<W>
where
    W: Write,
{
    fn new(writer: W) -> ByteWriter<W> {
        ByteWriter {
            writer,
            capacity: 0,
            slots: 0,
        }
    }
    fn write_bit(&mut self, bit: bool) -> Result<(), io::Error> {
        self.slots <<= 1;

        if bit {
            self.slots |= 1;
        }

        self.capacity += 1;

        if self.capacity == 8 {
            self.flush()?;
        }
        // println!("{}", self.capacity);
        Ok(())
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        let n = self.writer.write(&[self.slots])?;
        println!("Writing: {:0b} : {}", self.slots, n);
        self.capacity = 0;
        self.slots = 0;
        Ok(())
    }
    fn fill_to_flush(&mut self) -> Result<(), io::Error> {
        for _ in 0..8 - self.capacity {
            self.write_bit(false)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), io::Error> {
    something();
    Ok(())
}

fn something() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fs;
    use std::rc::Rc;

    #[derive(Debug, Clone)]
    struct TreeNode {
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
        value: Option<(Option<u8>, u32)>,
    }

    type Dictionary = Rc<RefCell<HashMap<u8, Package<u16>>>>;

    impl TreeNode {
        fn populate_tree(&self, path: Package<u16>, dict: Dictionary) {
            if let Some(branch) = &self.left {
                branch.populate_tree(
                    Package::new(path.content << 1, path.length + 1),
                    Rc::clone(&dict),
                );
            }
            if let Some(branch) = &self.right {
                branch.populate_tree(
                    Package::new(path.content << 1 | 0x1, path.length + 1),
                    Rc::clone(&dict),
                );
            }
            if let Some((Some(_), _)) = &self.value {
                dict.as_ref()
                    .borrow_mut()
                    .insert(self.value.unwrap().0.unwrap(), path);
            }
        }
    }

    // fn encode(dict: Dictionary, mut to_be_encoded: String) -> String {
    //     for (k, v) in dict.borrow_mut().iter() {
    //         to_be_encoded = to_be_encoded.replace(*k as char, v);
    //     }
    //     to_be_encoded
    // }
    fn encode_bytes(dict: &Dictionary, mut reader: impl Read) -> Result<(), io::Error> {
        let mut buf: [u8; 1] = [0];
        let mut bw = ByteWriter::new(File::create("encoded.huff")?);
        while let Ok(_) = reader.read_exact(&mut buf) {
            match dict.borrow().get(&buf[0]) {
                Some(thing) => {
                    println!(
                        "found match: {:x} will be replaced with: {:b} length: {}",
                        buf[0], thing.content, thing.length
                    );
                    let mut temp = thing.clone();
                    for _ in 0..thing.length {
                        bw.write_bit((temp.content & 0x1) != 0)?;
                        temp.content >>= 1;
                    }
                    println!("Added {:b} to the stack!", thing.content);
                }
                None => panic!("shouldnt happen, dict key not found"),
            }
        }
        bw.fill_to_flush()?;

        Ok(())
    }

    fn decode(dict: Dictionary, mut reader: impl Read, key: TreeNode) -> Result<(), io::Error> {
        let mut fw = File::create("decoded.txt")?;
        let mut buf: [u8; 1] = [0];
        let mut node = key.clone();
        while let Ok(_) = reader.read_exact(&mut buf) {
            let mut byte = buf[0];
            let mut i = 8;
            for _ in 0..i {
                println!("byte & 1: {}, byte: {}", byte & 1, byte);
                match byte & 1 {
                    0 => match node.left {
                        Some(_) => {
                            node = *node.left.unwrap();
                            byte >>= 1;
                        }
                        None => {
                            fw.write(&[node.value.unwrap().0.unwrap()]);
                            node = key.clone();
                            i += 1;
                        }
                    },
                    1 => match node.right {
                        Some(_) => {
                            node = *node.right.unwrap();
                            byte >>= 1;
                        }
                        None => {
                            fw.write(&[node.value.unwrap().0.unwrap()]);
                            node = key.clone();
                            i += 1;
                        }
                    },
                    _ => {}
                }
            }
        }

        Ok(())
    }

    let message = "Your message goes here.";

    println!("Original message: {}", message);

    let mut byte_occurencies_map: HashMap<u8, u32> = HashMap::new();

    let mut reader = File::open("./example").unwrap();
    let mut buf: [u8; 1] = [0];
    while let Ok(_) = reader.read_exact(&mut buf) {
        byte_occurencies_map
            .entry(buf[0])
            .and_modify(|o| *o += 1)
            .or_insert(1);
    }

    let mut occuriencies_feed: Vec<TreeNode> = vec![];

    for (k, v) in byte_occurencies_map {
        occuriencies_feed.push(TreeNode {
            left: None,
            right: None,
            value: Some((Some(k), v)),
        });
    }

    occuriencies_feed.sort_by(|a, b| b.value.unwrap().1.partial_cmp(&a.value.unwrap().1).unwrap());

    while occuriencies_feed.len() != 1 {
        let first = occuriencies_feed.pop().unwrap();
        let second = occuriencies_feed.pop().unwrap();

        let node = TreeNode {
            value: Some((None, first.value.unwrap().1 + second.value.unwrap().1)),
            left: Some(Box::new(first)),
            right: Some(Box::new(second)),
        };

        occuriencies_feed.push(node);
        occuriencies_feed
            .sort_by(|a, b| b.value.unwrap().1.partial_cmp(&a.value.unwrap().1).unwrap());
    }
    // println!("{:#?}", occuriencies_feed);

    let dict = Rc::new(RefCell::new(HashMap::new()));
    occuriencies_feed[0].populate_tree(Package::new(0, 0), Rc::clone(&dict));

    println!("Dictionary: {:#x?}", dict);
    let encoded = encode_bytes(&dict, File::open("./example").unwrap()).unwrap();
    // println!("Encoded message: {}", encoded);
    decode(
        dict,
        File::open("chuj.huff").unwrap(),
        occuriencies_feed[0].clone(),
    )
    .unwrap();
    // // println!("Decoded message: {}", decode(Rc::clone(&dict), encoded));
}
