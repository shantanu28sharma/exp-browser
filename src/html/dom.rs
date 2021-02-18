use std::vec::Vec;
use std::collections::BTreeMap;
use std::str;

#[derive(Debug)]
pub struct Node{
    node: NodeType
}

#[derive(Debug)]
pub enum NodeType{
    Text(String),
    Tag(TagNode)
}

#[derive(Debug)]
pub struct TagNode{
    name: String,
    children: Vec<Node>,
    attr: BTreeMap<String, String>
}

struct Parser{
    pos: usize,
}

impl Parser {
     pub fn new() -> Self {
        Self{pos: 0}
    }
    pub fn parse(&mut self, input: &str) -> Node {
        let input: Vec<u8> = input.bytes().collect();
        loop {
            if self.pos >= input.len(){
                break;
            }
            match input[self.pos] {
                b'<' => {
                    let (name, map) = Self::get_tag_info(self, &input);
                    let children = Self::get_children(self, &input, &name);
                    let node = Node{node: NodeType::Tag(TagNode{name, children, attr: map})};
                    return node;
                },
                _ => {
                    let node = Node{node: NodeType::Text(Self::match_until(self, &input, &vec![b'<']))};
                    return node;
                }
            }
        }
        todo!() 
    }
    fn get_tag_info(&mut self, input: &Vec<u8>) -> (String, BTreeMap<String, String>){
        Self::consume(self, input, b'<');
        let name = Self::match_until(self, input, &vec![b' ', b'>']);
        Self::consume_zero_plus(self, input, &vec![b' ']);
        let mut map : BTreeMap<String, String>= BTreeMap::new();
        while input[self.pos] != b'>' {
            let key = Self::match_until(self, input, &vec![b'=', b' ']);
            Self::consume_zero_plus(self, input, &vec![b' ']);
            Self::consume(self, input, b'=');
            Self::consume_zero_plus(self, input, &vec![b' ']);
            Self::consume(self, input, b'"');
            let value = Self::match_until(self, input, &vec![b'"']);
            Self::consume(self, input, b'"');
            map.insert(key, value);
            Self::consume_zero_plus(self, input, &vec![b' ']);
        }
        Self::consume(self, input, b'>');
        return (name, map);
    }
    fn get_children(&mut self, input: &Vec<u8>, _name: &str) -> Vec<Node> {
        let mut ret : Vec<Node> = vec![];
        while self.pos < input.len() {
            match input[self.pos] {
                b'<' => {
                    if input[self.pos+1] == b'/' {
                        Self::match_until(self, input, &vec![b'>']);
                        Self::consume(self, input, b'>');
                        break;
                    }
                    else {
                        let (name, map) = Self::get_tag_info(self, input);
                        let children = Self::get_children(self, input, &name);
                        let node = Node{node: NodeType::Tag(TagNode{name, children, attr: map})};
                        ret.push(node);
                    }
                },
                _ => {
                    let text = Self::match_until(self, input, &vec![b'<']);
                    ret.push(Node{node: NodeType::Text(text)});
                }
            }
        }
        return ret;
    }
    fn consume_zero_plus(&mut self, input: &Vec<u8>, chars: &Vec<u8>) {
        loop{
            let mut found: bool = false;
             for chr in  chars {
                if *chr==input[self.pos]{
                    self.pos+=1;
                    found = true;
                }
            }
            if !found{
                break;
            }
        }
    }
    fn match_until(&mut self, input: &Vec<u8>, chars: &Vec<u8>) -> String {
        let init = self.pos;
        loop{
            if self.pos >= input.len(){
                return str::from_utf8(&input[init..self.pos]).unwrap().to_string();
            }
            for chr in chars {
                if *chr==input[self.pos] {
                    return str::from_utf8(&input[init..self.pos]).unwrap().to_string();
                }
            }
            self.pos+=1;
        }
    }
    fn consume_option(&mut self, input: &Vec<u8>, chr: u8) {
        if self.pos <=input.len() && input[self.pos] == chr {
            self.pos+=1;
        }
    }
    fn consume(&mut self, input: &Vec<u8>, chr: u8){
        if input[self.pos] == chr {
            self.pos+=1;
        }
        else{
            panic!("Cannot match token {:?} at {:?}", chr as char, self.pos);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parser_check_text(){
        let input = r#"abcd"#;
        let mut parser = Parser::new();
        assert_eq!(format!("{:?}", parser.parse(&input)), r#"Node { node: Text("abcd") }"#);
    }
    #[test]
    fn parser_check_with_attributes(){
        let input = r#"<a id="something" class="something"></a>"#; 
        let mut parser = Parser::new();
        assert_eq!(format!("{:?}", parser.parse(&input)), r#"Node { node: Tag(TagNode { name: "a", children: [], attr: {"class": "something", "id": "something"} }) }"#);
    }
    #[test]
    fn children_test(){
        let input = r#"<html><div></div><a></a></html>"#;
        let mut parser = Parser::new();
        assert_eq!(format!("{:?}", parser.parse(&input)), r#"Node { node: Tag(TagNode { name: "html", children: [Node { node: Tag(TagNode { name: "div", children: [], attr: {} }) }, Node { node: Tag(TagNode { name: "a", children: [], attr: {} }) }], attr: {} }) }"#);
    }
}
