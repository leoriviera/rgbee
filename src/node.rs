use crate::colour::Colour;
use crate::quantiser::MAX_DEPTH;

#[derive(Default)]
pub struct Node {
    pub colour: Colour,
    pub pixel_count: usize,
    pub palette_index: usize,
    pub children: [Option<Box<Node>>; 8],
    pub level: usize,
}

impl Node {
    pub fn is_leaf(&self) -> bool {
        self.pixel_count > 0
    }

    pub fn get_colour(&self) -> Colour {
        Colour {
            red: self.colour.red / self.pixel_count,
            green: self.colour.green / self.pixel_count,
            blue: self.colour.blue / self.pixel_count,
        }
    }

    pub fn get_leaf_nodes(&self) -> Vec<&Box<Node>> {
        let mut leaf_nodes = Vec::new();

        for node in self.children.iter().flatten() {
            if node.is_leaf() {
                leaf_nodes.push(node)
            } else {
                leaf_nodes.append(&mut node.get_leaf_nodes())
            }
        }

        leaf_nodes
    }

    pub fn remove_leaves(&mut self) {
        for node in self.children.iter().flatten() {
            self.colour.add(&node.colour);
            self.pixel_count += node.pixel_count;
        }
    }

    pub fn add_colour(&mut self, colour: &Colour) {
        if self.level >= MAX_DEPTH - 1 {
            self.colour.add(colour);
            self.pixel_count += 1;
        } else {
            let index = Node::get_color_index_for_level(colour, self.level);

            if self.children[index].is_none() {
                let node = Box::new(Node {
                    level: self.level + 1,
                    ..Default::default()
                });

                self.children[index] = Some(node);
            }

            self.children[index].as_mut().unwrap().add_colour(colour);
        }
    }

    pub fn get_color_index_for_level(colour: &Colour, level: usize) -> usize {
        let mut index = 0;
        let mask = 0b10000000 >> level;

        if colour.red & mask != 0 {
            index |= 0b100;
        }

        if colour.green & mask != 0 {
            index |= 0b010;
        }

        if colour.blue & mask != 0 {
            index |= 0b001;
        }

        index
    }
}
