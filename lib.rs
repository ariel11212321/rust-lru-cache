use std::collections::HashMap;
use std::boxed::Box;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::fmt::Debug;

#[derive(Debug)]
struct KeyRef<K> {
  key: *const K,
}

impl <K: Hash> Hash for KeyRef<K> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    unsafe { (*self.key).hash(state) }
  }
}

impl <K: PartialEq> PartialEq for KeyRef<K> {
  fn eq(&self, other: &Self) -> bool {
    unsafe { (*self.key).eq(&*other.key) }
  }
}

impl <K: Eq> Eq for KeyRef<K> {}

#[derive(Debug)]
struct CacheEntry<K, V> {
  key: K,
  val: V,
  prev: *mut CacheEntry<K, V>,
  next: *mut CacheEntry<K, V>,  
}

impl <K: Default, V: Default> Default for CacheEntry<K, V> {
  fn default() -> Self {
    CacheEntry {
      key: Default::default(),
      val: Default::default(),
      prev: ptr::null_mut(),
      next: ptr::null_mut(),
    }
  }
}

struct LRUCacheBase<K, V> {
  map: HashMap<KeyRef<K>, Box<CacheEntry<K, V>>>,
  head: *mut CacheEntry<K, V>,
  capacity: usize,
}

impl<K, V> LRUCacheBase<K, V> where K: Default+Hash+Eq+Debug, V: Default+Hash+Debug  {
  pub fn new(capacity: usize) -> Self {
    let mut cache = LRUCacheBase {
      map: HashMap::new(),
      head: Box::into_raw(Box::new(Default::default())),
      capacity: capacity,
    };
    unsafe {
      (*cache.head).next = cache.head;
      (*cache.head).prev = cache.head;
    }
    return cache;
  }
  
  pub fn get<'a>(&'a mut self, key: &K) -> Option<&'a V> {
    match self.map.get_mut(&KeyRef{key: key}) {
      Some(node) => {
        let entry: *mut CacheEntry<K, V> = &mut **node;
        self.remove_node(entry);
        self.add_node(entry);
        return unsafe { Some(&(*entry).val) }
      },
      None => return None,
    }
  }
  
  pub fn put(&mut self, key: K, value: V) {
    match self.map.get_mut(&KeyRef{key: &key}) {
      Some(node) => {
        let entry: *mut CacheEntry<K, V> = &mut **node;
        self.remove_node(entry);
        self.add_node(entry);
        unsafe { (*entry).val = value; }
      },
      None => {
        if self.map.len() == self.capacity {
          unsafe {
            let entry: *mut CacheEntry<K, V> = (*self.head).prev;
            self.remove_node(entry);
            self.map.remove(&KeyRef{key: &(*entry).key});
          }
        }
        let mut entry: Box<CacheEntry<K, V>> = Box::new(Default::default());
        (*entry).val = value;
        (*entry).key = key;
        self.add_node(&mut *entry);
        self.map.insert(KeyRef{key: &(*entry).key}, entry);
      },
    }
  }

  fn add_node(&mut self, node: *mut CacheEntry<K, V>) {
    unsafe {
      (*node).next = (*self.head).next;
      (*node).prev = self.head;
      (*(*self.head).next).prev = node;
      (*self.head).next = node;
    }
  }

  fn remove_node(&mut self, node: *mut CacheEntry<K, V>) {
    unsafe {
      (*(*node).next).prev = (*node).prev;
      (*(*node).prev).next = (*node).next;
    }
  }

}

impl<K, V> Drop for LRUCacheBase<K, V> {
  fn drop(&mut self) {
    unsafe {
        Box::from_raw(self.head);
    }
  }
}

struct LRUCache {
  cache: LRUCacheBase<i32, i32>,
}



impl LRUCache {
    fn new(capacity: i32) -> Self {
      LRUCache {
        cache: LRUCacheBase::new(capacity as usize),
      }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        self.cache.get(&key).copied().unwrap_or(-1)
    }
    
    fn put(&mut self, key: i32, value: i32) {
        self.cache.put(key, value);
    }
}