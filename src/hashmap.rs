use std::borrow::Borrow;

trait KHash: Eq + Clone {
    fn hash(&self) -> u32;
}

enum Slot<K, V> {
    Zero,
    One((K, V)),
    Many(Vec<(K, V)>),
}

struct KMap<K, V> where K: KHash {
    slots: Vec<Slot<K, V>>,
    load: usize,
}

impl<K, V> KMap<K, V>
    where K: KHash,
          V: Clone {
    fn new() -> KMap<K, V> {
        let mut slots = Vec::<Slot<K, V>>::with_capacity(10);
        for _ in 0..10 {
            slots.push(Slot::Zero);
        }

        KMap { slots, load: 0 }
    }

    fn insert(&mut self, key: K, value: V) {
        // TODO: load factor reallocation.

        let target_slot = key.hash() as usize % self.slots.len();

        if let Slot::Zero = self.slots[target_slot] {
            self.slots[target_slot] = Slot::One((key, value));
        } else if let Slot::One(ref mut tuple) = &mut self.slots[target_slot] {
            if tuple.0 == key {
                tuple.1 = value
            } else {
                let new_vec = vec![tuple.clone(), (key, value)];
                self.slots[target_slot] = Slot::Many(new_vec)
            }
        } else if let Slot::Many(ref mut vec) = &mut self.slots[target_slot] {
            match vec.iter_mut().find(|(k, _)| *k == key) {
                None => vec.push((key, value)),
                Some((k, v)) => *v = value,
            }
        }
    }

    fn get<Q>(&self, key: &Q) -> Option<&V>
        where K: Borrow<Q>,
              Q: KHash + ?Sized
    {
        let slot_id = key.hash() as usize % self.slots.len();

        match &self.slots[slot_id] {
            Slot::Zero => None,
            Slot::One((k, v)) => {
                if k.borrow() == key {
                    Some(&v)
                } else {
                    None
                }
            }
            Slot::Many(vec) => {
                vec.iter().find(|(k, _)| k.borrow() == key).map(|(_, v)| v)
            }
        }
    }
}

impl KHash for String {
    fn hash(&self) -> u32 {
        // A truly bad hashing algo that maxes out as 256 slots.
        self.bytes().reduce(|a, e| a.overflowing_mul(e).0).unwrap() as u32
    }
}

#[test]
fn test_basic_hashmap() {
    let mut map = KMap::new();

    map.insert("apple".to_owned(), 123);
    map.insert("banana".to_owned(), 123);
    map.insert("apple".to_owned(), 124);
    map.insert("paple".to_owned(), 222); // Hash collision with "apple".

    assert_eq!(map.get(&"charlie".to_owned()), None);
    assert_eq!(map.get(&"apple".to_owned()), Some(&124));
    assert_eq!(map.get(&"paple".to_owned()), Some(&222));
    assert_eq!(map.get(&"banana".to_owned()), Some(&123));
}

// TODO: Load factor testing.