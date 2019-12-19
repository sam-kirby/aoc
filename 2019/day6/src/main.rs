use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Body {
    body_id: String,
    parent: Option<Rc<RefCell<Body>>>,
    children: Vec<Weak<RefCell<Body>>>, // Care more about traversing from leaves to root so use `Weak` here
}

impl Body {
    fn new(body_id: String) -> Body {
        Body {
            body_id,
            parent: None,
            children: Vec::new(),
        }
    }

    fn with_parent(body_id: String, parent: Rc<RefCell<Body>>) -> Body {
        Body {
            body_id,
            parent: Some(parent),
            children: Vec::new(),
        }
    }

    fn parent(&self) -> Option<Rc<RefCell<Body>>> {
        if let Some(parent) = &self.parent {
            Some(parent.clone())
        } else {
            None
        }
    }

    fn children(&self) -> Vec<Rc<RefCell<Body>>> {
        self.children
            .iter()
            .filter_map(|child| {
                if let Some(child) = child.upgrade() {
                    Some(child)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct OrbitTree {
    bodies: HashMap<String, Rc<RefCell<Body>>>,
}

impl OrbitTree {
    fn new() -> OrbitTree {
        let body = Body::new(String::from("COM"));
        let mut bodies = HashMap::new();
        bodies.insert(String::from("COM"), Rc::new(RefCell::new(body)));
        OrbitTree { bodies }
    }

    fn add_body(&mut self, body: Body) -> Result<(), String> {
        let parent_id = if let Some(parent) = body.parent() {
            parent.borrow().body_id.clone()
        } else {
            return Err(String::from("Body does not have a parent"));
        };

        if !self.bodies.contains_key(&parent_id) {
            return Err(String::from("Parent is not in the tree"));
        }

        let body_id = body.body_id.clone();
        let body_ptr = Rc::new(RefCell::new(body));

        body_ptr
            .borrow_mut()
            .parent()
            .unwrap()
            .borrow_mut()
            .children
            .push(Rc::downgrade(&body_ptr));
        self.bodies.insert(body_id, body_ptr);

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open(Path::new("input6.txt"))?;

    let mut string_buffer = String::new();
    file.read_to_string(&mut string_buffer)?;
    let mut orbit_list = string_buffer.lines().collect::<Vec<_>>();

    let mut orbit_tree = OrbitTree::new();

    while !orbit_list.is_empty() {
        let initial_length = orbit_list.len();

        orbit_list = orbit_list
            .iter()
            .cloned()
            .filter(|orbit| {
                let pair = orbit.split(')').collect::<Vec<_>>();
                match orbit_tree.bodies.entry(String::from(pair[0])) {
                    Entry::Occupied(entry) => {
                        let parent = entry.get().clone();
                        let body = Body::with_parent(String::from(pair[1]), parent);

                        orbit_tree.add_body(body).expect("Failed to add body");

                        false
                    }
                    Entry::Vacant(_) => true,
                }
            })
            .collect::<Vec<_>>();

        if initial_length == orbit_list.len() {
            panic!("No items were added!");
        }
    }

    if let Some(com) = orbit_tree.bodies.get("COM") {
        let mut orbits = 0;
        let mut depth = 0;
        let mut bodies_at_current_depth = vec![com.clone()];

        while !bodies_at_current_depth.is_empty() {
            bodies_at_current_depth = bodies_at_current_depth
                .iter()
                .flat_map(|elem| elem.borrow().children())
                .collect::<Vec<_>>();
            depth += 1;
            orbits += depth * bodies_at_current_depth.len();
        }

        println!("There are {} direct and indirect orbits", orbits);
    }

    let santa = match orbit_tree.bodies.entry(String::from("SAN")) {
        Entry::Occupied(santa) => santa.get().clone(),
        Entry::Vacant(_) => panic!("\"santa\" is not in the tree"),
    };
    let you = match orbit_tree.bodies.entry(String::from("YOU")) {
        Entry::Occupied(you) => you.get().clone(),
        Entry::Vacant(_) => panic!("\"you\" is not in the tree"),
    };

    let list_ancestors = |node: Rc<RefCell<Body>>| {
        let mut list = Vec::new();
        let mut parent_opt = node.borrow().parent();
        loop {
            match parent_opt {
                Some(parent) => {
                    list.push(parent.borrow().body_id.clone());
                    parent_opt = parent.borrow().parent()
                }
                None => {
                    break;
                }
            }
        }
        list.reverse();
        list
    };

    let santa_ancestors = list_ancestors(santa);
    let you_ancestors = list_ancestors(you);
    let first_divergent_ancestor = santa_ancestors
        .iter()
        .zip(you_ancestors.iter())
        .position(|elem| elem.0 != elem.1);
    match first_divergent_ancestor {
        Some(first_divergent_ancestor) => {
            let mut jump_dests =
                you_ancestors[first_divergent_ancestor - 1..you_ancestors.len() - 1].to_vec();
            jump_dests.reverse();
            let mut to_santa = santa_ancestors[first_divergent_ancestor..].to_vec();
            jump_dests.append(&mut to_santa);

            let mut jump_origins = you_ancestors[first_divergent_ancestor - 1..].to_vec();
            jump_origins.reverse();
            let mut origins_to_santa =
                santa_ancestors[first_divergent_ancestor..santa_ancestors.len() - 1].to_vec();
            jump_origins.append(&mut origins_to_santa);

            let jumps = jump_origins
                .iter()
                .zip(jump_dests.iter())
                .map(|elem| format!("{} --> {}", elem.0, elem.1))
                .collect::<Vec<_>>();

            for jump in &jumps {
                println!("{}", jump);
            }
            println!("You have to make {} jumps to reach Santa", jumps.len());
        }
        None => unreachable!("No common ancestors in a tree with a common root"),
    };

    Ok(())
}
