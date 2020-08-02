/**
 * This is some sample code written in the ++ programming language.
 */

@ Tree {
    constructor(value, children) {
        ^.value = value;
        (children)? {
            ^.children = children;
        }: {
            ^.children = [];
        }
    }
    
    print_all() {
        console.log(^.value);
        (child : ^.children)! {
            child.print_all();
        }
    }
}

// tree t is constant
$$t = #Tree(1, [#Tree(2), #Tree(3, [#Tree(4)])]);
t.print_all();
