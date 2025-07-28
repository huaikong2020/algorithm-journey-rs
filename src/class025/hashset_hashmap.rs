use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let p1 = Person::new(String::from("Alice"), 20);
        let p2 = Person::new(String::from("Bob"), 25);
        let p3 = Person::new(String::from("Charlie"), 30);

        let mut set1 = BTreeSet::new();
        set1.insert(&p1);
        set1.insert(&p2);
        set1.insert(&p3);
        assert_eq!(set1.len(), 3);
        assert!(set1.contains(&p1));
        assert!(set1.contains(&p2));
        assert!(set1.contains(&p3));
    }

    #[test]
    fn test_hashset() {
        let p1 = Person::new(String::from("Alice"), 20);
        let mut set1 = HashSet::new();
        set1.insert(&p1);
        assert_eq!(set1.len(), 1);
        assert!(set1.contains(&p1));

        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        set.remove(&2);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&1));
        assert!(!set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_hashmap() {
        let mut map = HashMap::new();
        explain("empty", &map);

        map.insert('a', 1);
        explain("added 1", &map);

        map.insert('b', 2);
        map.insert('c', 3);
        explain("added 3", &map);

        map.insert('d', 4);
        explain("added 4", &map);

        // get 时需要使用引用，并且也返回引用
        assert_eq!(map.get(&'a'), Some(&1));
        assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));

        map.remove(&'a');
        // 删除后就找不到了
        assert_eq!(map.contains_key(&'a'), false);
        assert_eq!(map.get(&'a'), None);
        explain("removed", &map);
        // shrink 后哈希表变小
        map.shrink_to_fit();
        explain("shrinked", &map);
    }
}

fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("{}: len: {}, cap: {}", name, map.len(), map.capacity());
}
