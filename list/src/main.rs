mod linked_list;

fn main() {
    let l: &mut linked_list::LinkedList<i32> = &mut linked_list::LinkedList::new();
    l.push(1);
    l.push(2);
    l.push(3);
    l.push(4);
    l.push(5);

    loop {
        let element = l.pop();
        if element.is_none() {
            break;
        }

        println!("{}", element.unwrap());
    }
}