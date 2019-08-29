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
                        Rule::ordered_list_item
                        | Rule::unordered_list_item
                        | Rule::labeled_list_item => {
                            let (level, typ, content) = get_level_type_content(e_in.clone());
                            let current_item = ListItem {
                                typ,
                                level,
                                children: vec![],
                                content, //e_in.as_str().to_string(),
                            };
                            if item_list.contains(&current_item) {
                                loop {
                                    let stack_top_item = item_list.pop().unwrap();
                                    if stack_top_item == current_item && item_list.len() > 0 {
                                        // attach the stack top item to its previous node
                                        // push current node to stack
                                        if item_list.contains(&stack_top_item) {
                                            // 如果栈中还是有和当前 item 相同的 item，说明这些 item 本来也是并列的，不需要弹出了
                                            item_list.push(stack_top_item);
                                            item_list.push(current_item);
                                            break;
                                        }
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
                                // 先 push
                                // 遍历完之后
                                // 需要从后向前扫描一次栈，如果栈中元素的 level/type 不相等
                                // 需要把 child attach 到 parent 的 children 数组中去
                                item_list.push(current_item);
                            }
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

    // attach the last nodes to their parent
    while item_list.len() > 1 {
        let top = item_list.pop().unwrap();
        if item_list.contains(&top) {
            item_list.push(top);
            break;
        } else {
            item_list.last_mut().unwrap().children.push(top);
        }
    }

    println!("xx{:#?}", item_list);
    String::new()
}

// TODO
fn get_level_type_content(e: Pair<Rule>) -> (i8, ListItemType, String) {
    match e.as_rule() {
        /*
        ordered_list_item = {
            element_attributes?
            ~ ordered_list_item_prefix
            ~ ordered_list_item_content
        }
        */
        Rule::ordered_list_item => {
            for e_in in e.into_inner() {
                match e_in.as_rule() {
                    Rule::element_attributes => {} // TODO
                    Rule::ordered_list_item_prefix => {
                        // .{1,}  => level = counter
                        // 1. 2.  => level = -1
                        // a. b.  => level = -2
                        // A. B.  => level = -3
                    }
                    Rule::ordered_list_item_content => {}
                    _ => unreachable!(),
                }
            }
        }
        /*
        unordered_list_item = {
            element_attributes?
            ~ unordered_list_item_prefix
            ~ unordered_list_item_check_style?
            ~ unordered_list_item_content
        }
        */
        Rule::unordered_list_item => {
            for e_in in e.into_inner() {
                match e_in.as_rule() {
                    Rule::element_attributes => {} // todo
                    Rule::unordered_list_item_prefix => {}
                    Rule::unordered_list_item_check_style => {}
                    Rule::unordered_list_item_content => {}
                    _ => unreachable!(),
                }
            }
        }
        /*
        labeled_list_item = {
            element_attributes?
            ~ labeled_list_item_term
            ~ labeled_list_item_separator
            ~ labeled_list_item_description?
        }
        */
        Rule::labeled_list_item => {
            for e_in in e.into_inner() {
                match e_in.as_rule() {
                    Rule::element_attributes => {} // TODO
                    Rule::labeled_list_item_term => {}
                    Rule::labeled_list_item_separator => {}
                    Rule::labeled_list_item_description => {}
                    _ => unreachable!(),
                }
            }
        }
        _ => unreachable!(),
    }
    (0, ListItemType::OrderedItem, String::new())
}