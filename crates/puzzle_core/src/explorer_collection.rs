use std::sync::atomic;

#[derive(Debug, Clone)]
pub struct ExplorerObject {
    name: String,
    id: ExplorerId,
    expanable: Option<(Vec<ExplorerObject>, bool)>,
}

#[derive(Debug, Clone, Copy)]
pub struct ExplorerId {
    value: usize,
}

static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
impl ExplorerId {
    fn new() -> ExplorerId {
        ExplorerId {
            value: COUNTER.fetch_add(1, atomic::Ordering::Relaxed),
        }
    }
}

/*#[derive(Debug)]
pub struct ExplorerFolder<'a> {
    children: &'a ExplorerObject,
    expanded: &'a bool,
}*/

impl ExplorerObject {
    #[must_use]
    pub fn new(name: String) -> ExplorerObject {
        ExplorerObject {
            name,
            id: ExplorerId::new(),
            expanable: None,
        }
    }
    #[must_use]
    pub fn build_folder(name: String, paths: &Vec<Vec<String>>) -> ExplorerObject {
        let mut new = ExplorerObject::new(name);
        for path in paths {
            new.push_path(path);
        }
        new
    }
    pub fn push(&mut self, object: ExplorerObject) {
        if let Some((children, _expanded)) = &mut self.expanable {
            children.push(object);
        } else {
            self.expanable = Some((vec![object], false));
        }
    }
    pub fn push_path(&mut self, path: &[String]) {
        if let Some((first, rest)) = path.split_first() {
            if let Some((object, _)) = &mut self.expanable {
                if let Some(preexisting) = object.iter_mut().find(|a| &a.name == first) {
                    preexisting.push_path(rest);
                } else {
                    let mut new = ExplorerObject::new(first.clone());
                    new.push_path(rest);
                    object.push(new);
                }
            } else {
                let mut new = ExplorerObject::new(first.clone());
                new.push_path(rest);
                self.push(new);
            }
        }
    }
    #[must_use]
    pub fn id(&self) -> ExplorerId {
        self.id
    }
    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }
    #[must_use]
    pub fn expanded(&self) -> Option<(&Vec<ExplorerObject>, bool)> {
        if let Some((children, expanded)) = &self.expanable {
            Some((children, *expanded))
        } else {
            None
        }
    }
    pub fn expand(&mut self, value: bool) {
        if let Some((_, expanded)) = &mut self.expanable {
            *expanded = value;
        }
    }
    #[must_use]
    pub fn get_by_id_mut(&mut self, id: ExplorerId) -> Option<&mut ExplorerObject> {
        if self.id.value == id.value {
            Some(self)
        } else if let Some((objects, _)) = &mut self.expanable {
            objects.iter_mut().find_map(|x| x.get_by_id_mut(id))
        } else {
            None
        }
    }
    #[must_use]
    pub fn get_by_id(&self, id: ExplorerId) -> Option<&ExplorerObject> {
        if self.id.value == id.value {
            Some(self)
        } else if let Some((objects, _)) = &self.expanable {
            objects.iter().find_map(|x| x.get_by_id(id))
        } else {
            None
        }
    }
}
