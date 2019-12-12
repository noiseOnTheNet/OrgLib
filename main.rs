
mod org;
use org::{Node,Status};
 

fn add_child2(parent: &mut Node, child : Node){
    &parent.children.push(child);
}
fn main() {
  let mut x = Node::new("hi".to_string());
  let mut y = Node::new("there".to_string());
  let mut z = Node::new("how".to_string());
  let mut w = Node::new("are".to_string());
  y.add_child(w);
  add_child2(&mut x,y);
  add_child2(&mut x,z);
  x.status = Some(Status::TODO);
  println!("Debug {:?}", x);
  println!("Display {}", x);
}
