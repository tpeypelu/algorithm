use algorithm::list::List;
use algorithm::list::SingleLinkedList;

#[test]
fn create_single_linked_list() {
    let value = 7;

    let list = SingleLinkedList::new(value);

    assert_eq!(*list.value(), value);

    match list.next() {
        Some(_) => assert!(false),
        None => assert!(true),
    }
}

#[test]
fn merge_lists() {
    let value1 = 2;
    let value2 = 3;

    let mut list = SingleLinkedList::new(value1);
    let list_to_add = SingleLinkedList::new(value2);

    list.add(list_to_add);

    assert_eq!(*list.value(), value1);

    match list.next() {
        None => assert!(false),
        Some(x) => assert_eq!(*x.value(), value2),
    }
}
