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
                            let (level, typ, need_checkbox, title, description) =
                                get_level_type_content(e_in.clone());
                            let current_item = ListItem {
                                typ,
                                level,
                                children: vec![],
                                title,
                                description,
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
                                .description
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
fn get_level_type_content(e: Pair<Rule>) -> (i8, ListItemType, bool, String, String) {
    let (mut typ, mut level, mut need_checkbox, mut title, mut description) = (
        ListItemType::OrderedItem,
        0,
        false,
        String::new(),
        String::new(),
    );
    match e.as_rule() {
        Rule::ordered_list_item => {
            typ = ListItemType::OrderedItem;
            for e_in in e.into_inner() {
                match e_in.as_rule() {
                    Rule::element_attributes => {} // TODO
                    Rule::ordered_list_item_prefix => {
                        // .{1,}  => level = counter
                        // 1. 2.  => level = -1
                        // a. b.  => level = -2
                        // A. B.  => level = -3
                        let prefix = e_in.as_str();
                        match prefix.chars().nth(0).unwrap() {
                            '.' => {
                                level = prefix.trim().chars().count() as i8;
                            }
                            '0'..='9' => {
                                level = -1;
                            }
                            'a'..='z' => {
                                level = -2;
                            }
                            'A'..='Z' => {
                                level = -3;
                            }
                            _ => unreachable!(),
                        }
                    }
                    Rule::ordered_list_item_content => {
                        title.push_str(e_in.as_str());
                    }
                    _ => unreachable!(),
                }
            }
        }
        Rule::unordered_list_item => {
            typ = ListItemType::UnorderedItem;
            for e_in in e.into_inner() {
                match e_in.as_rule() {
                    Rule::element_attributes => {} // todo
                    Rule::unordered_list_item_prefix => {
                        // * => counter
                        // - => -1
                        let prefix = e_in.as_str().trim();
                        match prefix.chars().nth(0).unwrap() {
                            '-' => {
                                level = -1;
                            }
                            '*' => {
                                level = prefix.chars().count() as i8;
                            }
                            _ => unreachable!(),
                        }
                    }
                    Rule::unordered_list_item_check_style => {
                        need_checkbox = true;
                    }
                    Rule::unordered_list_item_content => {
                        // todo
                        title.push_str(e_in.as_str());
                    }
                    _ => unreachable!(),
                }
            }
        }
        Rule::labeled_list_item => {
            typ = ListItemType::LabeledItem;
            for e_in in e.into_inner() {
                match e_in.as_rule() {
                    Rule::element_attributes => {} // TODO
                    Rule::labeled_list_item_term => {
                        title = e_in.as_str().to_string();
                    }
                    Rule::labeled_list_item_separator => {
                        level = e_in.as_str().trim().chars().count() as i8;
                    }
                    Rule::labeled_list_item_description => {
                        description.push_str(e_in.as_str());
                    }
                    _ => unreachable!(),
                }
            }
        }
        _ => unreachable!(),
    }
    (level, typ, need_checkbox, title, description)
}
