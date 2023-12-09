use crate::{colour::Colour, node::Node};

pub const MAX_DEPTH: usize = 8;

#[derive(Default)]
pub struct Quantiser {
    pub root: Node,
}

impl Quantiser {
    pub fn get_leaves(&self) -> Vec<&Box<Node>> {
        self.root.get_leaf_nodes()
    }

    pub fn add_colour(&mut self, colour: &Colour) {
        self.root.add_colour(colour);
    }

    pub fn get_nodes_at_level(&mut self, level: usize) -> Vec<&mut Node> {
        fn helper(node: &mut Node, level: usize) -> Vec<&mut Node> {
            if node.level == level {
                vec![node]
            } else {
                let mut nodes = Vec::new();

                for node in node.children.iter_mut().flatten() {
                    let node_reference = node;

                    nodes.extend(helper(node_reference, level))
                }

                nodes
            }
        }

        helper(&mut self.root, level)
    }

    pub fn make_palette(&mut self, colour_count: usize) -> Vec<Colour> {
        for level in (1..=MAX_DEPTH).rev() {
            for node in self.get_nodes_at_level(level) {
                node.remove_leaves()
            }

            if self.get_leaves().len() <= colour_count {
                break;
            }
        }

        let mut palette = Vec::new();

        for (palette_index, node) in self.get_leaves().into_iter().enumerate() {
            if palette_index >= colour_count {
                break;
            }

            if node.is_leaf() {
                palette.push(node.get_colour());
            }
        }

        palette
    }

    // pub fn add_level_node(&mut self, level: usize, node: &'a Box<Node>) {
    //     let mut copied = node.clone();

    //     // self.levels[level].push(&mut copied);
    // }

    // pub fn make_palette(&self, colour_count: usize) -> Vec<Colour> {
    //     let mut palette = Vec::new();

    //     let palette_indez = 0;

    //     let mut leaf_count = self.get_leaves().len();

    //     for level in self.levels.iter() {
    //         if !level.is_empty() {
    //             for node in level {
    //                 leaf_count -= node.remove_leaves();
    //             }
    //         }
    //     }

    //     palette
    // }
}
