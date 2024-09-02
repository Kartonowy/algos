use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    value: Option<(Option<char>, u32)>,
}

type Dictionary = Rc<RefCell<HashMap<char, String>>>;

impl TreeNode {
    fn populate_dict(&self, path: String, dict: Dictionary) {
        if let Some(branch) = &self.left {
            branch.populate_dict(path.clone() + "0", Rc::clone(&dict));
        }
        if let Some(branch) = &self.right {
            branch.populate_dict(path.clone() + "1", Rc::clone(&dict));
        }
        if let Some((Some(_), _)) = &self.value {
            dict.borrow_mut()
                .insert(self.value.unwrap().0.unwrap(), path);
        }
    }
}

fn encode(dict: Dictionary, mut to_be_encoded: String) -> String {
    for (k, v) in dict.borrow_mut().iter() {
        to_be_encoded = to_be_encoded.replace(*k, v);
    }
    to_be_encoded
}

fn decode(dict: Dictionary, encoded_message: String) -> String {
    let mut clump = "".to_string();
    let mut result = encoded_message.clone();


    for character in encoded_message.chars() {
        match clump.len() {
            0 => clump = character.to_string(),
            1.. => clump += &character.to_string(),
        }
        for (k, v) in dict.borrow_mut().iter() {
            if &clump == v {
                result = result.replacen(&clump, &k.to_string(), 1).to_string();
                clump = "".to_string()
            }
        }
    }

    result
}

fn main() {
    let message = "Your message goes here.";

    println!("Original message: {}", message);

    let mut occurencies_map: HashMap<char, u32> = HashMap::new();

    for character in message.chars() {
        occurencies_map
            .entry(character)
            .and_modify(|o| *o += 1)
            .or_insert(1);
    }

    let mut occuriencies_feed: Vec<TreeNode> = vec![];

    for (k, v) in occurencies_map {
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

    let dict = Rc::new(RefCell::new(HashMap::new()));

    occuriencies_feed[0].populate_dict("".to_string(), Rc::clone(&dict));

    println!("Dictionary: {:#?}", dict);
    let encoded = encode(Rc::clone(&dict), message.to_string());
    println!("Encoded message: {}", encoded);
    println!("Decoded message: {}", decode(Rc::clone(&dict), encoded));
}
