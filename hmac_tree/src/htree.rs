use std::borrow::Borrow;
use std::ops::{Index, IndexMut};
use std::vec;
use crate::encoders::hex_rep::decode_hex;
use crate::encoders::serializer::HtreeJsonSerializer;
use crate::hmac::Hmac;
use crate::encoders::index_parser::IndexType;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct HTreeNode {
    id: u32,
    pub left: Option<Arc<Mutex<HTreeNode>>>,
    pub right: Option<Arc<Mutex<HTreeNode>>>,
    pub digest: Vec<u8>,
    pub data: Vec<u8>,
    pub index: String,
    
}

pub enum InsertBranch {
    Left(Option<Arc<Mutex<HTreeNode>>>),
    Right(Option<Arc<Mutex<HTreeNode>>>),
}

impl HTreeNode {
    fn new(id: u32, data: Vec<u8>, digest: Vec<u8>, index: String) -> Self {
        HTreeNode { id, data, digest, left: None, right: None, index }
    }

    fn new_root(data: Vec<u8>, digest: Vec<u8>) -> Self {
        HTreeNode { id: 0, left: None, right: None, digest, data, index: "ROOT".to_string() }
    }

    fn new_empty() -> Self {
        HTreeNode { id: 0, left: None, right: None, digest: vec![], data: vec![], index: "".to_string() }
    }
 
    pub fn set_left(&mut self, left: Option<Arc<Mutex<HTreeNode>>>) {
        self.left = left;
    }

    pub fn set_right(&mut self, right: Option<Arc<Mutex<HTreeNode>>>) {
        self.right = right;
    }

    pub fn validate_bytes(&self, digest: Vec<u8>, other_digest: Vec<u8>) -> bool {
        let hmac_self = Hmac::from_bytes(self.data.clone(), digest);

        hmac_self.valitate_bytes(other_digest)
    }

    pub fn validate_str(&self, digest: &str, other_digest: &str) -> bool {
        let hmac_self = Hmac::from_str(
                String::from_utf8(self.data.clone()).unwrap().as_str(), 
                digest);

        hmac_self.valitate_str(other_digest)
    }

    pub fn get_index(&self, index: String) -> Option<Arc<Mutex<Self>>> {
        let index_parsed: IndexType = index.into();

        let node = match index_parsed {
            IndexType::Root => Some(Arc::new(Mutex::new(self.clone()))),
            IndexType::Mixed(v) => {
            let n = {
                let mut ni = None;

                for (l, r, g) in v {                   
                    let nx = match g {
                        'l' => {
                            let mut nii = self.clone().left;
                            let mut i = 0usize;
                            let mut j = 0usize;

                            while let Some(n) = nii {
                                let n_deref = &*n;
                                let n_lock = n_deref.lock().unwrap();

                                nii = n_lock.left.to_owned();
    
                                i += 1;
    
                                if i >= l {
                                    break;
                                }
                            }

                            while let Some(n) = nii {
                                let n_deref = &*n;
                                let n_lock = n_deref.lock().unwrap();

                                nii = n_lock.right.to_owned();
    
                                j += 1;
    
                                if j >= l {
                                    break;;
                                }
                            }
    
                            nii                       
    
                        },
                        'r' => {
                            let mut nii = self.clone().left;
                            let mut i = 0usize;
                            let mut j = 0usize;

                            while let Some(n) = nii {
                                let n_deref = &*n;
                                let n_lock = n_deref.lock().unwrap();

                                nii = n_lock.right.to_owned();
    
                                i += 1;
    
                                if i >= l {
                                    break;
                                }
                            }

                            while let Some(n) = nii {
                                let n_deref = &*n;
                                let n_lock = n_deref.lock().unwrap();
                               
                                nii = n_lock.left.to_owned();
    
                                j += 1;
    
                                if j >= l {
                                    break;;
                                }
                            }
    
                            nii                       
    
                        },
                        _ => panic!("Wrong index"),
                    };  

                    ni = nx;
                };
                
                ni
            };
                n                
            },
            IndexType::OnlyLeft(v) => {   
                let mut nii = self.clone().left;

                
                for l in v {   
  
                    let mut i = 0usize;    
                        
                    while let Some(n) = nii {
                        let n_deref = &*n;
                        let n_lock = n_deref.lock().unwrap();

                        nii = n_lock.left.to_owned();
        
                        i += 1;
        
                        if i >= l {
                            break;
                        }
                    }
                }  
                                          
                nii
            },
            IndexType::OnlyRight(v) => {   
                let mut nii = self.clone().left;

                
                for l in v {   
  
                    let mut i = 0usize;    
                        
                    while let Some(n) = nii {
                        let n_deref = &*n;
                        let n_lock = n_deref.lock().unwrap();

                        nii = n_lock.right.to_owned();
        
                        i += 1;
        
                        if i >= l {
                            break;
                        }
                    }
                }  
                                          
                nii
            },
        };


        node

    }

    pub fn insert(&self, path: String, item: InsertBranch) {
        let node = self.get_index(path);

        match node {
            Some(n) => {
                let n_deref = &*n;
                let mut n_mut_lock = n.lock().unwrap();

                match item {
                    InsertBranch::Left(i) => {
                        n_mut_lock.left = i;
                    },
                    InsertBranch::Right(i) => {
                        n_mut_lock.right = i;
                    },
                }
            },
            None => (),
        }

       
    }

    pub fn get_in_order_recurse(node: Option<Arc<Mutex<HTreeNode>>>, v: &mut Vec<Arc<Mutex<HTreeNode>>>) {
        match node {
            Some(n) => {
                let n_deref = &*n;
                let n_lock = n_deref.lock().unwrap();

                let n_left = n_lock.left.to_owned();
                let n_right = n_lock.right.to_owned();

                Self::get_in_order_recurse(n_left, v);

                v.push(n.clone());

                Self::get_in_order_recurse(n_right, v);
            }, 
            None => (),
        }
    }

    pub fn get_pre_order_recurse(node: Option<Arc<Mutex<HTreeNode>>>, v: &mut Vec<Arc<Mutex<HTreeNode>>>) {
        match node {
            Some(n) => {
                let n_deref = &*n;
                let n_lock = n_deref.lock().unwrap();

                let n_left = n_lock.left.to_owned();
                let n_right = n_lock.right.to_owned();

                v.push(n.clone());

                Self::get_in_order_recurse(n_left, v);

                Self::get_in_order_recurse(n_right, v);
            }, 
            None => (),
        }
    }

    pub fn get_post_order_recurse(node: Option<Arc<Mutex<HTreeNode>>>, v: &mut Vec<Arc<Mutex<HTreeNode>>>) {
        match node {
            Some(n) => {
                let n_deref = &*n;
                let n_lock = n_deref.lock().unwrap();

                let n_left = n_lock.left.to_owned();
                let n_right = n_lock.right.to_owned();

                Self::get_in_order_recurse(n_left, v);

                Self::get_in_order_recurse(n_right, v);

                v.push(n.clone());

            }, 
            None => (),
        }
    }

    

}


pub struct Htree<T: HtreeJsonSerializer + Clone> {
    data_containers: Vec<T>,
    root: Arc<Mutex<HTreeNode>>,
}

impl<T: HtreeJsonSerializer + Clone> Htree<T> {
    fn new(data: Vec<T>, key: &str) {

        let key_bytes = key.as_bytes().to_vec();

        let mut root = Self::make_node(
            data.get(0).unwrap(),
            0, 
            key_bytes.clone());


        data[1..]
                    .to_vec()
                    .iter()
                    .step_by(2)
                    .enumerate()
                    .map(|(i, x)| {
                        let left = Some(Self::make_node(
                            x, 
                            i as u32, 
                            key_bytes.clone()));
                        
                        let right = match data.get(i + 1) {
                            Some(t) => {
                                Some(Self::make_node(t, 
                                    i as u32, key_bytes.clone()))
                            },

                            None => None,
                        };

                        


                    });

    }

    fn make_node(t: &T, id: u32, key: Vec<u8>) -> Arc<Mutex<HTreeNode>> {
        let data = t.ser_into_json();
        let hmac = Hmac::from_bytes(data.clone(), key);
        let digest = hmac.calculate().0;


        Arc::new(Mutex::new(HTreeNode::new_empty()))
    }
}