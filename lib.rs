use std::{collections::HashMap};
use std::hash::Hash;
use std::ops::Deref;
use crate::Node::{Cons, NULL}; 

#[derive(Clone, Debug, PartialEq)]
pub enum Node<T> 
{
    Cons(T, Option<Box<Node<T>>>),
    NULL
}
impl<T> Deref for Node<T> 
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Cons(x, _) => x,
            NULL => panic!("NULL dereferenced")
        }
    }
}

impl<T: Clone> Node<T> 
{   
    pub fn new(x: T) -> Self {
        Cons(x, None)
    }
    pub fn new_with_next(x: T, next: Box<Node<T>>) -> Self {
        Cons(x, Some(next))
    }
    pub fn set_next_raw(&mut self, next: Box<Node<T>>) {
        match self {
            Cons(_, ref mut next_mut) => *next_mut = Some(next),
            NULL => panic!("NULL dereferenced")
        }
    }

    pub fn set_next(&mut self, next_data: T) 
    {
        match self {
            Cons(_, next) => {
                *next = Some(Box::new(Cons(next_data, None)));
            }
            NULL => panic!("NULL dereferenced")
        }
    }

    pub fn get_next(&self) -> &Option<Box<Node<T>>> 
    {
        match self {
            Cons(_, next) => {
                next
            }
            _ => &None
        }
    }
    pub fn set_value(&mut self, value: T) 
    {
        match self {
            Cons(_, _) => {
                *self = Cons(value, None);
            }
            NULL => panic!("NULL dereferenced")
        }
    }
    pub fn get_value(&self) -> &T 
    {
        match self {
            Cons(x, _) => x,
            NULL => panic!("NULL dereferenced")
        }
    }
    pub fn not_null(&self) -> bool {
        match self {
            NULL => false,
            _ => true
        }
    }
    

}


pub struct LRUCache<T: Clone + Hash + Eq> 
{
    len: usize,
    capacity: usize,
    cache: HashMap<T, Box<Node<T>>>,
    head: Box<Node<T>>,
    tail: Box<Node<T>>,
}
impl<T: Clone + Hash + Eq> LRUCache<T> 
{
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            len: 0,
            capacity,
            cache: HashMap::new(),
            head: Box::new(NULL),
            tail: Box::new(NULL)
        }
    }
    pub fn put(&mut self, key: T, value: T) {
        self.cache.insert(key, Box::new(Node::new(value)));
    }

    pub fn get(&mut self, key: &T) -> &T {
        return self.cache.get(key).unwrap().get_value();
    }
    pub fn get_mut(&mut self, key: &T) -> Option<&mut Box<Node<T>>> {
        return self.cache.get_mut(key);
    }
    pub fn insert(&mut self, key: &T) -> Option<&mut Box<Node<T>>> {
        let new_node = Box::new(Node::new(key.clone()));
        self.cache.insert(key.clone(), new_node);
        self.len += 1;
        return self.cache.get_mut(key);
    }
    pub fn remove(&mut self, key: &T) -> Option<Box<Node<T>>> {
        let node = self.cache.remove(key);
        if node.clone().is_some() {
            self.len -= 1;
            self.remove_from_head(node.clone().unwrap());
        }
        return node;
    }
    pub fn remove_from_head(&mut self, node: Box<Node<T>>) {
        if self.head.get_next().is_some() {
            self.head.set_next_raw(node.clone());
        } else {
            self.tail = node;
        }
    }
    pub fn add_to_head(&mut self, data: &T) {
        if self.head.get_next().is_none() {
            self.tail = Box::new(Node::new(data.clone()));
        }
        

    }
    

    
}




#[test]
fn test() 
{
    let mut t: LRUCache<i32> = LRUCache::new(10);

   for i in 0..10 
   {
        t.put(i, i);
        println!("{:?}", t.cache);
    }
   

}
fn main() 
{
    
}