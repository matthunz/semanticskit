use accesskit::{Node, NodeBuilder, NodeClassSet, Rect};
use slotmap::{DefaultKey, SlotMap};
use taffy::{prelude::Layout, style::Style, Taffy};

pub struct Element<T> {
    node_builder: NodeBuilder,
    layout: Layout,
    children: Vec<DefaultKey>,
    pub data: T
}

pub struct Tree<T> {
    taffy: Taffy,
    elements: SlotMap<DefaultKey, Element<T>>,
    classes: NodeClassSet,
    root: DefaultKey,
}

impl<T> Tree<T> {
    pub fn update_style(&mut self, key: DefaultKey, style: Style) {
        self.taffy.set_style(key, style).unwrap();
    }

    pub fn update_semantics(&mut self, key: DefaultKey) -> Option<Node> {
        let layout = self.taffy.layout(key).unwrap();
        let element = self.elements.get_mut(key).unwrap();

        let mut is_dirty = false;
        if layout.location != element.layout.location || layout.size != element.layout.size {
            element.node_builder.set_bounds(Rect::new(
                layout.location.x as _,
                layout.location.y as _,
                (layout.location.x + layout.size.width) as _,
                (layout.location.y + layout.size.height) as _,
            ));
            is_dirty = true;
            element.layout = layout.clone();
        }

        if is_dirty {
            let node = element.node_builder.clone().build(&mut self.classes);
            Some(node)
        } else {
            None
        }
    }

    pub fn visit_mut(&mut self, visitor: &mut impl VisitMut<T>) {
        let mut keys = vec![self.root];
        while let Some(key) = keys.pop() {
            let element = self.elements.get_mut(key).unwrap();
            visitor.visit_element(element);
            keys.extend_from_slice(&element.children);
        }
    }
}

pub trait VisitMut<T> {
    fn visit_element(&mut self, element: &mut Element<T>);
}
