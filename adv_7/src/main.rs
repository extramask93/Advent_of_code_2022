use std::fs;

use regex::Regex;
use slab_tree::{NodeId, Tree, TreeBuilder};

pub trait Executable {
    fn run(&self, ft: &mut FileTree);
}
pub trait Matcher {
    fn try_match(&self, s: &str) -> Option<Box<dyn Executable>>;
}
#[derive(Debug)]
struct ChangeDirCmd {
    d: String,
}
#[derive(Debug)]
struct ListCmd {}
#[derive(Debug)]
struct FileEntry {
    size: u32,
    name: String,
}
#[derive(Debug)]
struct DirEntry {
    name: String,
}
struct FileMatcher {}
struct DirEntryMatcher {}
struct ListCmdMatcher {}
struct ChangeDirCmdMatcher {}
impl Matcher for ChangeDirCmdMatcher {
    fn try_match(&self, s: &str) -> Option<Box<dyn Executable>> {
        let re = Regex::new(r"\$ cd (.*)").unwrap();
        re.is_match(&s).then(|| {
            Box::new(ChangeDirCmd {
                d: re
                    .captures(&s)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_string(),
            }) as Box<dyn Executable>
        })
    }
}
impl Executable for ListCmd {
    fn run(&self, _ft: &mut FileTree) {
        println!("{:?}", self);
    }
}
impl Matcher for ListCmdMatcher {
    fn try_match(&self, s: &str) -> Option<Box<dyn Executable>> {
        (s == "$ ls").then(|| Box::new(ListCmd {}) as Box<dyn Executable>)
    }
}
impl Executable for ChangeDirCmd {
    fn run(&self, ft: &mut FileTree) {
        println!("{:?}", self);
        if self.d == r"/" {
            ft.current_id = ft.tree.root_id().expect("no root id");
        } else if self.d == ".." {
            ft.current_id = ft
                .tree
                .get(ft.current_id)
                .unwrap()
                .ancestors()
                .next()
                .unwrap()
                .node_id();
        } else {
            ft.current_id = ft
                .tree
                .get(ft.current_id)
                .unwrap()
                .children()
                .filter(|node| node.data().name == self.d)
                .last()
                .unwrap()
                .node_id();
        }
    }
}
impl Matcher for FileMatcher {
    fn try_match(&self, s: &str) -> Option<Box<dyn Executable>> {
        let re = Regex::new(r"(\d+) ([[:alpha:]]+)").unwrap();
        re.is_match(&s).then(|| {
            Box::new(FileEntry {
                size: re
                    .captures(&s)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
                name: re
                    .captures(&s)
                    .unwrap()
                    .get(2)
                    .unwrap()
                    .as_str()
                    .to_string(),
            }) as Box<dyn Executable>
        })
    }
}
impl Executable for FileEntry {
    fn run(&self, ft: &mut FileTree) {
        println!("{:?}", self);
        ft.tree.get_mut(ft.current_id).unwrap().append(Entry {
            name: self.name.to_string(),
            is_dir: false,
            size: self.size,
        });
    }
}
impl Executable for DirEntry {
    fn run(&self, ft: &mut FileTree) {
        println!("{:?}", self);
        ft.tree.get_mut(ft.current_id).unwrap().append(Entry {
            name: self.name.to_string(),
            is_dir: true,
            size: 0,
        });
    }
}
impl Matcher for DirEntryMatcher {
    fn try_match(&self, s: &str) -> Option<Box<dyn Executable>> {
        let re = Regex::new(r"dir ([[:alpha:]]+)").unwrap();
        re.is_match(s).then(|| {
            Box::new(DirEntry {
                name: re
                    .captures(s)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_string(),
            }) as Box<dyn Executable>
        })
    }
}
struct MatcherRegistry {
    matchers: Vec<Box<dyn Matcher>>,
}

impl MatcherRegistry {
    fn new() -> Self {
        Self { matchers: vec![] }
    }
    fn register_matcher(&mut self, matcher: impl Matcher + 'static) {
        self.matchers.push(Box::new(matcher));
    }
    fn try_get_cmd(&self, s: &str) -> Option<Box<dyn Executable>> {
        self.matchers
            .iter()
            .filter_map(|matcher| matcher.try_match(s))
            .next()
    }
}
#[derive(Debug)]
struct Entry {
    is_dir: bool,
    name: String,
    size: u32,
}
pub struct FileTree {
    current_id: NodeId,
    tree: Tree<Entry>,
}
fn apply_cmd(ft: &mut FileTree, e: &Box<dyn Executable>) {
    e.run(ft);
}
fn main() -> anyhow::Result<()> {
    let mut registry = MatcherRegistry::new();
    let mut tr = TreeBuilder::new()
        .with_root(Entry {
            is_dir: true,
            name: r"\".to_string(),
            size: 0,
        })
        .build();
    let mut ft = FileTree {
        current_id: tr.root_id().unwrap(),
        tree: tr,
    };
    registry.register_matcher(FileMatcher {});
    registry.register_matcher(DirEntryMatcher {});
    registry.register_matcher(ListCmdMatcher {});
    registry.register_matcher(ChangeDirCmdMatcher {});
    fs::read_to_string("/home/damian/rust/advofcode2022/adv_7/input.txt")?
        .split('\n')
        .map(|l| l.trim())
        .filter_map(|line| registry.try_get_cmd(line))
        .for_each(|e| apply_cmd(&mut ft, &e));
    let dirs: Vec<_> = ft
        .tree
        .root()
        .unwrap()
        .traverse_level_order()
        .filter(|node| node.data().is_dir)
        .collect();
    let sum: u32 = dirs
        .iter()
        .map(|dr| {
            dr.traverse_level_order()
                .fold(0, |acc, entry| entry.data().size + acc)
        })
        .filter(|&s| s <= 100000)
        .sum();
    println!("part1: {sum}");
    let disc_size = 70000000;
    let min_disc_size_required = 30000000;
    let total_size_on_disc: u32 = dirs
        .iter()
        .map(|dr| {
            dr.traverse_level_order()
                .fold(0, |acc, entry| entry.data().size + acc)
        })
        .max()
        .unwrap();
    let sum2: u32 = dirs
        .iter()
        .map(|dr| {
            dr.traverse_level_order()
                .fold(0, |acc, entry| entry.data().size + acc)
        })
        .filter(|&s| s >= min_disc_size_required - (disc_size - total_size_on_disc))
        .min()
        .unwrap();
    println!("part2: {sum2}");
    //traverse each and filter files
    //count files sizes
    Ok(())
}
