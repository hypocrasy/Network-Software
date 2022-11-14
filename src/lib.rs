// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

use std::path::{Path, PathBuf};
use tokio::io::*;


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_to_file() {
    //     let mes = b"chenbo\r\nis god";
    //     let res = File {name: "chenbo".to_string(), content: "is god".as_bytes().to_vec()};
    //     assert_eq!(res, to_file(mes).unwrap());
    // }
    #[test]
    fn test_split() {
        let s = String::from_utf8(vec![0u8; 1024]).unwrap();
        println!("{} {}", s.len(), s);
    }

}

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct File {
    name: String,
    content: Vec<u8>
}

enum Method {
    Get,
    Post,
    List,
    Uninitialized,
}

impl Default for Method {
    fn default() -> Self {
        Self::Get
    }
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "LIST" => Method::List,
            _ => Method::Uninitialized
        }
    }
}

type Resource = PathBuf;


// $staredWithAu32
// GET /usr/cb/file $content.len() info HCCB
// $content

#[derive(Default)]
struct HccbRequest {
    method: Method,
    path: Resource,
    content: Vec<u8>
}

struct HccbResponse<'a> {
    status: &'a str,
    status_text: &'a str,
    content: Vec<u8>
}

// impl From<&[u8]> for HccbRequest {
//     fn from(s: &[u8]) -> Self {
//         let hccbRequest = HccbRequest::default();
        
//     }
// }

#[derive(Default)]
pub struct AuthKey {
    _inner: Vec<u32>,
    _inner_size: Vec<u32>
}

impl AuthKey {
    pub fn insert_u16(&mut self, n: u16) {
        self._inner.push(n as u32);
        self._inner_size.push(2);
    }    

    pub fn insert_u32(&mut self, n:u32) {
        self._inner.push(n as u32);
        self._inner_size.push(4);
    }

    pub async fn send(&self, tx: &mut (impl AsyncWrite+Unpin)) {
        assert_eq!(self._inner, self._inner_size);
        tx.write_u32(self._inner.len() as u32).await.unwrap();
        for i in 0..self._inner.len() {
            tx.write_u32(self._inner_size[i]).await.unwrap();
        }
        for i in 0..self._inner.len() {
            if self._inner_size[i] == 4 {
                tx.write_u32(self._inner[i]).await.unwrap();
            } else if self._inner_size[i] == 2 {
                tx.write_u16(self._inner[i] as u16).await.unwrap();
            } else {
                panic!();
            }
        }
        
    }

}

pub async fn syn_decode(rx: &mut (impl AsyncRead+Unpin)) -> Option<AuthKey> {
    let mut v = vec![];
    let mut sz = vec![];
    let num = rx.read_u32().await.unwrap();
    for _ in 0..num {
        sz.push(rx.read_u32().await.unwrap());
    }
    for i in 0..num {
        if sz[i as usize] == 4 {
            v.push(rx.read_u32().await.unwrap());
        } else if sz[i as usize] == 2 {
            v.push(rx.read_u16().await.unwrap() as u32);
        } else {
            return None;
        }
    }
    Some(AuthKey { _inner: v, _inner_size: sz })
}

