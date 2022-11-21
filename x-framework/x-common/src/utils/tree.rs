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
    fn get_children(&self, id: &str) -> Self;
    fn to_tree(&self) -> Self;
}

impl<T> Tree<T> for Vec<T>
where
    T: Treeable<T> + Clone,
{
    fn get_children(&self, id: &str) -> Vec<T> {
        self.iter().filter(|x| x.get_parent_id() == id).cloned().collect()
    }
    fn to_tree(&self) -> Vec<T> {
        let mut root = vec![];
        for item in self {
            if item.get_parent_id() == "0" {
                let children = self.get_children(item.get_id());
                if !children.is_empty() {
                    let mut item = item.to_owned();
                    item.set_children(children);
                    root.push(item);
                }
            }
        }
        root
    }
}
