use parking_lot::Mutex; use std::collections::VecDeque;
#[derive(Clone)] pub struct RingBuf<T>{inner:std::sync::Arc<Mutex<VecDeque<T>>>,cap:usize}
impl<T> RingBuf<T>{pub fn new(cap:usize)->Self{Self{inner:std::sync::Arc::new(Mutex::new(VecDeque::with_capacity(cap))),cap}}
pub fn push(&self,v:T){let mut g=self.inner.lock(); if g.len()==self.cap{g.pop_front();} g.push_back(v);}
pub fn snapshot(&self)->Vec<T> where T:Clone{self.inner.lock().iter().cloned().collect()}}
