
extern crate num;
extern crate treedisplay;

pub mod common;
pub mod simple;

type TreeItem = f64;

impl common::Metric<f64> for TreeItem {
    fn distance(self, rhs: f64) -> f64 {
        (rhs - self).abs() as f64
    }
}

impl<'a> common::Metric<f64> for &'a TreeItem {
    fn distance(self, rhs: f64) -> f64 {
        (rhs - self).abs() as f64
    }
}

impl<'b> common::Metric<&'b f64> for TreeItem {
    fn distance(self, rhs: &'b f64) -> f64 {
        (rhs - self).abs() as f64
    }
}

impl<'a, 'b> common::Metric<&'b f64> for &'a TreeItem {

    fn distance(self, rhs:&'b f64) -> f64 {
        (rhs - self).abs() as f64
    }
}

impl common::CoverTreeData for TreeItem {}

#[test]
fn test_simple_cover_tree() {
    use simple::CoverTree;
    use common::NearestNeighbor;
    let mut ct: CoverTree<TreeItem> = CoverTree::new();

    ct.insert(5f64);
    ct.insert(8f64);
    ct.insert(15f64);

    assert_eq!(ct.find_nearest(10f64), Some(&8f64));
}

