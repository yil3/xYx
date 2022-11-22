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
    fn to_tree(&self, parent_id: &str) -> Vec<T>;
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
    ///    User { id: "1".to_string(), parent_id: "0".to_string(), name: "1".to_string(), children: vec![] },
    ///    User { id: "2".to_string(), parent_id: "0".to_string(), name: "2".to_string(), children: vec![] },
    ///    User { id: "3".to_string(), parent_id: "1".to_string(), name: "3".to_string(), children: vec![] },
    ///    User { id: "4".to_string(), parent_id: "2".to_string(), name: "4".to_string(), children: vec![] },
    ///    ];
    ///    let tree = list.to_tree("0");
    ///    println!("{:?}", tree);
    ///    assert_eq!(tree.len(), 2);
    fn to_tree(&self, parent_id: &str) -> Vec<T> {
        let f = |pid: &str| -> Vec<T> {
            let mut v = self
                .iter()
                .filter(|x| x.get_parent_id() == pid)
                .cloned()
                .collect::<Vec<T>>();
            for e in v.iter_mut() {
                e.set_children(self.to_tree(e.get_id()));
            }
            v
        };
        f(parent_id)
    }
}
