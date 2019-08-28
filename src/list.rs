use crate::parse::*;
use crate::structs::*;
use pest::iterators::Pair;

pub fn list_items(ast: Pair<Rule>) -> String {
    let mut item_list: Vec<ListItem> = vec![];
    let mut pre_content = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::list_item => {
                if let Some(e_in) = e.into_inner().next() {
                    match e_in.as_rule() {
                        Rule::ordered_list_item => {
                            let current_item = ListItem {
                                typ: ListItemType::OrderedItem,
                                level: 0,
                                children: vec![],
                                content: e_in.as_str().to_string(),
                            };
                            if item_list.contains(&current_item) {
                                loop {
                                    let stack_top_item = item_list.pop().unwrap();
                                    if stack_top_item == current_item && item_list.len() > 0 {
                                        // attach the stack top item to its previous node
                                        // push current node to stack
                                        item_list.last_mut().unwrap().children.push(stack_top_item);
                                        item_list.push(current_item);
                                        break;
                                    }
                                    if stack_top_item == current_item && item_list.len() == 0 {
                                        // no parent, the stack top and the current should be the
                                        // same originals
                                        item_list.push(stack_top_item);
                                        item_list.push(current_item);
                                        break;
                                    }
                                    // item != current_item
                                    // attach the top item to its previous node
                                    item_list.last_mut().unwrap().children.push(stack_top_item);
                                }
                            } else {
                                item_list.push(ListItem {
                                    typ: ListItemType::OrderedItem,
                                    level: 0,
                                    children: vec![],
                                    content: "".to_string(),
                                });
                            }
                        }
                        Rule::unordered_list_item => {
                            // TODO
                        }
                        Rule::labeled_list_item => {
                            // TODO
                        }
                        Rule::continued_list_item_element => {
                            if item_list.is_empty() {
                                // in this case
                                // this item should be treat as normal line/paragraph
                                pre_content += e_in.as_str();
                                continue;
                            }
                            item_list
                                .last_mut()
                                .unwrap()
                                .content
                                .push_str(e_in.as_str());
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Rule::blank_line => {}
            _ => unreachable!(),
        }
    }
    String::new()
}
