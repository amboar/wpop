#[derive(Copy, Clone, Debug)]
pub struct PopularWord<'a> {
    word: Option<&'a str>,
    count: u32,
}

struct PopularState<'a> {
    first: PopularWord<'a>,
    second: PopularWord<'a>,
}

impl<'a> PopularState<'a> {
    fn update(&mut self, current: PopularWord<'a>) {
        if current.count > self.first.count {
            self.first = current;
            self.second = self.first;
        } else if current.count > self.second.count {
            self.second = current;
        }
    }
}

pub mod radix {
    use std::str::Chars;
    use std::cell::RefCell;
    use std::ops::DerefMut;
    use super::PopularWord;
    use super::PopularState;

    #[derive(Clone)]
    struct Node {
        count: u32,
        nodes: Vec<Option<Box<Node>>>,
    }

    macro_rules! mknode {
        () => {
            Box::new(Node { count: 0, nodes : vec![None; 26] })
        }
    }

    fn index(c: char) -> usize {
        (c as u32 - 'a' as u32) as usize
    }

    impl Node {
        fn insert(&mut self, i: usize) {
            match self.nodes[i] {
                None => {
                    self.nodes[i] = Some(mknode!());
                },
                _ => (),
            }
        }

        fn increment(&mut self) {
            self.count += 1;
        }
    }

    struct RadixState<'a> {
        sentence: &'a str,
        tail: Chars<'a>,
        i: usize,
        start: usize,
    }

    impl<'a> RadixState<'a> {
        fn insert(&mut self, char: Option<char>, node: &mut Node) -> PopularWord<'a> {
            match char {
                Some(c) => {
                    if ' ' == c {
                        node.increment();
                        let word = &self.sentence[self.start .. self.i];
                        self.start = self.i + 1;
                        self.i += 1;
                        PopularWord { word: Some(word), count: node.count }
                    } else {
                        self.i += 1;
                        let i = index(c);
                        node.insert(i);
                        match node.nodes[i] {
                            Some(ref mut next) => {
                                let c = self.tail.next();
                                self.insert(c, &mut *next)
                            },
                            _ => panic!("Boo"),
                        }
                    }
                },
                None => {
                    node.increment();
                    let word = &self.sentence[self.start .. ];
                    PopularWord { word: Some(word), count: node.count }
                },
            }
        }
    }

    struct WordRadix<'a> {
        root: RefCell<Node>,
        state: RefCell<RadixState<'a>>,
    }

    impl<'a> WordRadix<'a> {
        fn insert(&self) -> Option<PopularWord<'a>> {
            let mut s = self.state.borrow_mut();
            let oc = s.deref_mut().tail.next();
            if oc.is_some() {
                let mut r = self.root.borrow_mut();
                let mr = r.deref_mut();
                Some(s.insert(oc, mr))
            } else {
                None
            }
        }
    }

    pub fn wpop(sentence: &str) -> PopularWord {
        let radix = WordRadix {
            root: RefCell::new(Node { count: 0, nodes : vec![None; 26] }),
            state: RefCell::new(RadixState {
                sentence: sentence,
                tail: sentence.chars(),
                i: 0,
                start: 0,
            }),
        };
        let mut state = PopularState {
            first: PopularWord { word: None, count: 0 },
            second: PopularWord { word: None, count: 0 },
        };

        while let Some(word) = radix.insert() {
            state.update(word);
        }

        state.first
    }
}

pub mod hash {
    use std::collections::HashMap;
    use super::PopularWord;
    use super::PopularState;

    pub fn wpop(sentence: &str) -> PopularWord {
        let mut map = HashMap::new();
        let mut state = PopularState {
            first: PopularWord { word: None, count: 0 },
            second: PopularWord { word: None, count: 0 },
        };
        let mut start: usize = 0;

        for (i, char) in sentence.chars().enumerate() {
            match char {
                ' ' => {
                    let word = &sentence[start .. (i as usize)];
                    let count = map.entry(word).or_insert(0);
                    *count += 1;
                    let current = PopularWord { word: Some(word), count: *count };
                    state.update(current);
                    start = i + 1;
                },
                _ => (),
            }
        }
        let word = &sentence[start .. ];
        let count = map.entry(word).or_insert(0);
        *count += 1;
        let current = PopularWord { word: Some(word), count: *count };
        state.update(current);

        state.first
    }
}
