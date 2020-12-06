use super::*;
use std::ops::Deref;

impl HirContext {
    pub fn visit_item_list(&mut self, item_list: &[danube_ast::Item]) -> HirResult<()> {
        let mut function_list = vec![];
        for item in item_list.iter() {
            match item.deref() {
                danube_ast::ItemKind::Function(node) => function_list.push(node),
            }
        }

        for function in function_list {
            self.visit_function(function)?;
        }

        Ok(())
    }
}
