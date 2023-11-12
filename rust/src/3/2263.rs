use std::io::{ stdin, stdout, Write, BufWriter };

fn get_node_by_in_and_postorder<T: Write>(
    inorder_nodes: &Vec<u32>,
    postorder_nodes: &Vec<u32>,
    io_start: usize, io_end: usize,
    po_start: usize, po_end: usize,
    writer: &mut BufWriter<T>
) {
    let count = po_end - po_start;

    if count == 1 {
        write!(writer, "{} ", inorder_nodes[io_start]).unwrap();
        return;
        // return Some(TreeNode::new(inorder_nodes[io_start]));
    }

    if count == 0 {
        return;
    }

    // println!("{} {} {} {}", io_start, io_end, po_start, po_end);

    for left_anchor in 0..count {
        // -- make second anchor and check
        if inorder_nodes[io_start + left_anchor] != postorder_nodes[po_end-1] {
            continue;
        }

        /*
        println!("({} {} {} {}, {} {} {} {})",
            io_start, io_start + left_anchor, io_start + left_anchor + 1, io_end,
            po_start, po_start + left_anchor, po_end - left_anchor, po_end
        );
        */

        write!(writer, "{} ", postorder_nodes[po_end-1]).unwrap();

        // left
        get_node_by_in_and_postorder(
            inorder_nodes, postorder_nodes,
            io_start, io_start + left_anchor,
            po_start, po_start + left_anchor,
            writer
        );

        // right
        get_node_by_in_and_postorder(
            inorder_nodes, postorder_nodes,
            io_start + left_anchor + 1, io_end,
            po_start + left_anchor, po_end - 1,
            writer
        );
    }
}

fn main() {
    // -- get data from stdin
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let node_count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let inorder_nodes = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let postorder_nodes = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut writer = BufWriter::new(stdout());
    get_node_by_in_and_postorder(
        &inorder_nodes, &postorder_nodes,
        0, node_count,
        0, node_count,
        &mut writer
    );
}
