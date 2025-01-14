use crate::{
    db::RirDatabase,
    middle::context::tls::CUR_ITEM,
    rir::{Item, NodeKind},
    Plugin,
};

#[derive(Clone, Copy)]
pub struct _WorkspacePlugin;

impl Plugin for _WorkspacePlugin {
    fn on_codegen_uint(&mut self, cx: &crate::Context, _items: &[crate::DefId]) {
        cx.entry_map.iter().for_each(|(k, v)| {
            cx.plugin_gen.insert(k.clone(), "".to_string());
            v.iter().for_each(|(def_id, _)| {
                CUR_ITEM.set(def_id, || {
                    let node = cx.node(*def_id).unwrap();

                    match &node.kind {
                        NodeKind::Item(item) => self.on_item(cx, *def_id, item.clone()),
                        _ => {}
                    }
                });
            })
        });
    }

    fn on_item(
        &mut self,
        cx: &crate::Context,
        def_id: crate::DefId,
        item: std::sync::Arc<crate::rir::Item>,
    ) {
        match &*item {
            Item::Service(s) => {
                if let Some(loc) = cx.location_map.get(&def_id) {
                    if let Some(mut gen) = cx.plugin_gen.get_mut(loc) {
                        gen.push_str(&format!("pub struct {};", s.name.sym));
                    }
                };
            }
            _ => {}
        }
    }
}
