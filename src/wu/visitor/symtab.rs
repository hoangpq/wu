use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;

use super::visitor::*;


#[derive(Debug, Clone)]
pub struct Frame {
  pub table: RefCell<HashMap<String, Type>>,
  pub depth: usize,
}

impl Frame {
  pub fn new(depth: usize) -> Self {
    Frame {
      table: RefCell::new(HashMap::new()),
      depth,
    }
  }

  pub fn from(table: HashMap<String, Type>, depth: usize) -> Self {
    Frame {
      table: RefCell::new(table),
      depth,
    }
  }

  pub fn get(&self, name: &String) -> Option<Type> {
    if let Some(v) = self.table.borrow().get(name) {
      Some(v.clone())
    } else {
      None
    }
  }

  pub fn assign(&mut self, name: String, t: Type) {
    self.table.borrow_mut().insert(name, t);
  }

  pub fn debug(&self) {
    println!("======= frame @ {}", self.depth);
    for (name, t) in self.table.borrow().iter() {
      println!("{} = {}", name, t)
    }

    println!()
  }
}


#[derive(Debug, Clone)]
pub struct SymTab {
  pub stack:  Vec<Frame>, // active frames
  pub record: Vec<Frame>, // popped frames

  pub depth: usize,

  pub implementations: HashMap<String, HashMap<String, Type>>,
  pub foreign_imports: HashMap<String, HashMap<String, Type>>,
}

impl SymTab {
  pub fn new() -> Self {
    SymTab {
      stack:  vec!(Frame::new(0)),
      record: Vec::new(),

      depth: 0,

      implementations: HashMap::new(),
      foreign_imports: HashMap::new(),
    }
  }

  pub fn from(table: HashMap<String, Type>) -> Self {
    SymTab {
      stack:  vec!(Frame::from(table, 0)),
      record: Vec::new(),

      depth: 0,

      implementations: HashMap::new(),
      foreign_imports: HashMap::new(),
    }
  }



  pub fn assign(&mut self, name: String, t: Type) {
    self.current_frame_mut().assign(name, t)
  }

  pub fn assign_str(&mut self, name: &str, t: Type) {
    self.current_frame_mut().assign(name.to_string(), t)
  }



  pub fn fetch(&self, name: &String) -> Option<Type> {
    let mut offset = self.stack.len() - 1;

    loop {
      if let Some(t) = self.stack[offset].get(name) {
        return Some(t)
      } else {
        if offset == 0 {
          return None
        }

        offset -= 1;
      }
    }
  }

  pub fn fetch_str(&self, name: &str) -> Option<Type> {
    self.fetch(&name.to_string())
  }



  pub fn revert_frame(&mut self) {
    self.stack.push(self.record.pop().unwrap().clone());
  }



  pub fn current_frame(&self) -> &Frame {
    self.stack.last().unwrap()
  }

  pub fn current_frame_mut(&mut self) -> &mut Frame {
    self.stack.last_mut().unwrap()
  }



  pub fn put_frame(&mut self, frame: Frame) {
    self.stack.push(frame)
  }



  pub fn push(&mut self) {
    self.stack.push(Frame::new(self.depth))
  }

  pub fn pop(&mut self) {
    let popped = self.stack.pop().unwrap();

    self.record.push(popped)
  }

  pub fn enter(&mut self) {
    self.depth += 1
  }

  pub fn exit(&mut self) {
    if self.depth > 0 {
      self.depth -= 1
    }
  }



  pub fn get_implementations(&self, id: &String) -> Option<&HashMap<String, Type>>  {
    self.implementations.get(id)
  }

  pub fn get_implementation_force(&self, id: &String, method_name: &String) -> Type {
    self.get_implementations(id).unwrap().get(method_name).unwrap().clone()
  }

  pub fn implement(&mut self, id: &String, method_name: String, method_type: Type) {
    if let Some(ref mut content) = self.implementations.get_mut(id) {
      content.insert(method_name, method_type);

      return
    }

    let mut hash = HashMap::new();

    hash.insert(method_name, method_type);

    self.implementations.insert(id.to_owned(), hash);
  }


  pub fn get_foreign_module(&self, id: &String) -> Option<&HashMap<String, Type>>  {
    self.foreign_imports.get(id)
  }

  pub fn import(&mut self, id: String, origin: HashMap<String, Type>) {
    self.foreign_imports.insert(id, origin);
  }
}