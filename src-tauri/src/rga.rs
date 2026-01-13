use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Id {
    pub id: u32,
    pub device: u32,
}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.id.partial_cmp(&other.id) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.device.partial_cmp(&other.device)
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Rga {
    root: Id,
    nodes: HashMap<Id, Node>,
    device: u32,
    next_id: u32,
}

pub struct Node {
    pub char: char,
    pub is_deleted: bool,
    pub parent: Id,
    /// INVARIANT: must always be updated and sorted in descending order
    pub children: Vec<Id>,
}

impl Rga {
    pub fn new(device: u32) -> Self {
        let mut nodes = HashMap::new();
        let root = Id { id: 0, device };
        nodes.insert(
            root,
            Node {
                char: '\0',
                is_deleted: true,
                parent: root,
                children: Vec::new(),
            },
        );
        Self {
            root,
            nodes,
            device,
            next_id: 1,
        }
    }

    pub fn insert(&mut self, char: char, parent: Id) -> Id {
        let parent_node = &self.nodes[&parent];
        let max_node = parent_node
            .children
            .iter()
            .map(|id| id.id)
            .max()
            .unwrap_or(self.next_id - 1);
        let local_id = if max_node >= self.next_id {
            max_node + 1
        } else {
            self.next_id
        };
        self.next_id = local_id + 1;
        let id = Id {
            id: local_id,
            device: self.device,
        };
        self.nodes.get_mut(&parent).unwrap().children.push(id);

        self.nodes.insert(
            id,
            Node {
                char,
                is_deleted: false,
                parent,
                children: Vec::new(),
            },
        );

        id
    }

    pub fn delete(&mut self, id: Id) -> Id {
        self.nodes.get_mut(&id).unwrap().is_deleted = true;
        let node = &self.nodes[&id];
        let parent = node.parent;
        let sibling_id = self.nodes[&parent]
            .children
            .iter()
            .copied()
            .filter(|sibling| sibling.id > id.id)
            .min();

        sibling_id.unwrap_or(parent)
    }

    fn to_list_from(&self, parent: Id, list: &mut String) {
        for child in &self.nodes[&parent].children {
            let node = &self.nodes[&child];
            if !node.is_deleted {
                list.push(node.char);
            }
            self.to_list_from(*child, list);
        }
    }

    pub fn to_list(&self) -> String {
        let mut text = String::new();
        self.to_list_from(self.root, &mut text);
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_add() {
        let mut rga = Rga::new(0);
        let mut cursor = Id { id: 0, device: 0 };
        cursor = rga.insert('a', cursor);
        cursor = rga.insert('b', cursor);
        cursor = rga.insert('c', cursor);
        let _ = cursor;
        assert_eq!(rga.to_list(), "abc");
    }

    #[test]
    fn linear_add_del() {
        let mut rga = Rga::new(0);
        let mut cursor = Id { id: 0, device: 0 };
        cursor = rga.insert('a', cursor);
        cursor = rga.insert('b', cursor);
        cursor = rga.insert('c', cursor);
        cursor = rga.delete(cursor);
        let _ = cursor;
        assert_eq!(rga.to_list(), "ab");
    }

    #[test]
    fn weird_add() {
        let mut rga = Rga::new(0);
        let mut cursor = Id { id: 0, device: 0 };
        let cursor2 = rga.insert('a', cursor);
        let _ = rga.insert('b', cursor);
        cursor = rga.insert('c', cursor2);
        let _ = cursor;
        assert_eq!(rga.to_list(), "acb");
    }

    #[test]
    fn weird_add_del() {
        let mut rga = Rga::new(0);
        let cursor = Id { id: 0, device: 0 };
        let cursor2 = rga.insert('a', cursor);
        rga.insert('b', cursor);
        let _ = rga.insert('c', cursor2);
        rga.delete(cursor2);
        assert_eq!(rga.to_list(), "cb");
    }
}
