/**
* @Author xYx
* @Date 2022-11-21 16:11:53
*/
pub trait Treeable<T> {
    fn get_id(&self) -> &str;
    fn get_parent_id(&self) -> &str;
    fn set_children(&mut self, children: Vec<T>);
}

pub trait Tree<T> {
    fn to_tree(&self) -> Vec<T>;
    fn to_tree_with_parent(&self, parent_id: &str) -> Vec<T>;
}

pub struct TreeUtil<T: Treeable<T>> {
    pub data: Vec<T>,
}

impl<T: Treeable<T>> TreeUtil<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T: Treeable<T> + Clone> Tree<T> for TreeUtil<T> {
    fn to_tree(&self) -> Vec<T> {
        self.to_tree_with_parent("0")
    }

    fn to_tree_with_parent(&self, parent_id: &str) -> Vec<T> {
        to_tree_with_parent(&self.data, parent_id)
    }
}

pub fn to_tree_with_parent<T: Treeable<T> + Clone>(data: &Vec<T>, parent_id: &str) -> Vec<T> {
    let f = |pid: &str| -> Vec<T> {
        let v = &mut data
            .iter()
            .filter(|x| x.get_parent_id() == pid)
            .cloned()
            .collect::<Vec<T>>();
        for e in v.iter_mut() {
            e.set_children(to_tree(data));
        }
        v.to_vec()
    };
    f(parent_id)
}

pub fn to_tree<T: Treeable<T> + Clone>(data: &Vec<T>) -> Vec<T> {
    to_tree_with_parent(data, "0")
}

impl<T> Tree<T> for Vec<T>
where
    T: Treeable<T> + Clone,
{
    /// 递归转换为树形结构
    /// # Example
    /// ```
    /// use x_common::utils::vector::Tree;
    /// let mut list = vec![
    ///    User { id: "1".to_string(), parent_id: "0".to_string(), children: vec![] },
    ///    User { id: "2".to_string(), parent_id: "0".to_string(), children: vec![] },
    ///    User { id: "3".to_string(), parent_id: "1".to_string(), children: vec![] },
    ///    User { id: "4".to_string(), parent_id: "2".to_string(), children: vec![] },
    ///    ];
    ///    let tree = list.to_tree();
    ///    println!("{:?}", tree);
    ///    assert_eq!(tree.len(), 2);
    fn to_tree(&self) -> Vec<T> {
        self.to_tree_with_parent("0")
    }

    fn to_tree_with_parent(&self, parent_id: &str) -> Vec<T> {
        to_tree_with_parent(self, parent_id)
    }
}
