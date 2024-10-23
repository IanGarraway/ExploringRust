use std::collections::BinaryHeap;

fn main() {

    //heaps, a lot like stacks, but self sorts to put the highest value at the top/front

    let mut bheap = BinaryHeap::new();
    bheap.push(1);
    bheap.push(18);
    bheap.push(20);
    bheap.push(19);
    bheap.push(6);
    
    println!("Peek: {:?}", bheap.peek()); //lets you see the top value without removing it

    println!("pre-pop: {:?}", bheap);

    bheap.pop();

    println!("Post-pop{:?}", bheap);
    
}
