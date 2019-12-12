use std::fmt;
#[derive(Debug)]
pub enum Status {TODO , WIP , DONE}

fn display_option_status(s : &Option<Status>) -> &'static str{
  match *s {
    None => "",
    Some(Status::TODO) => "TODO ",
    Some(Status::WIP) => "WIP ",
    Some(Status::DONE) => "DONE ",
  }
}

#[derive(Debug)]
enum Month {
  Jan,
  Feb,
  Mar,
  Apr,
  May,
  Jun,
  Jul,
  Aug,
  Sep,
  Oct,
  Nov,
  Dec
}

#[derive(Debug)]
struct Date{
  year : u16,
  month : Month,
  day : u16
}

#[derive(Debug)]
struct Time{
  hour : u16,
  min : u16
}

#[derive(Debug)]
enum RepeatInterval {
  Year,
  Month,
  Week,
  Day
}

#[derive(Debug)]
struct Repeat {
  mult : u16,
  interval : RepeatInterval,
  skip : bool
}

#[derive(Debug)]
pub struct Timestamp{
  date : Date,
  time : Option<Time>,
  repeat : Option<Repeat>
}

#[derive(Debug)]
pub struct Node{
  pub title : String,
  pub scheduled : Option<Timestamp>,
  pub deadline : Option<Timestamp>,
  pub text : String,
  pub status : Option<Status>,
  pub children : Vec<Node>
}

impl Node{
  pub fn add_child(self: &mut Node, child : Node){
    &self.children.push(child);
  }
  pub fn format(self: &Node, level: usize) -> String{
    let subnodes = &self.children;
    let status = &self.status;
    let subtext: Vec<String> = subnodes.into_iter().map(|x| x.format(level + 1)).collect();
    format!(
      "{} {}{}\n{}",
      "*".repeat(level),
      display_option_status(status),
      self.title,subtext.join("")
      )
  }
  pub fn new(title : String) -> Node{
    Node{
      title: title,
      scheduled : None,
      deadline : None,
      text: String::from(""),
      status: None,
      children: Vec::new()
    }
  }
}

impl fmt::Display for Node {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.format(1))
    }
}
