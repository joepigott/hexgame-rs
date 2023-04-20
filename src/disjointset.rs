pub struct DisjointSet {
    nodes: Vec<isize>
}

impl DisjointSet {
    /** 
    * Returns a new DisjointSet with (size) elements.
    */
    pub fn new(size: usize) -> Self {
        let mut result: Vec<isize> = Vec::new();

        for _ in 0..size {
            result.push(-1);
        }

        Self {
            nodes: result
        }
    }
    
    /** 
    * Joins the two sets (node1) and (node2) are found in. Uses smart-union by 
    * size.
    */
    pub fn union(&mut self, node1: usize, node2: usize) {
        let root1 = self.find(&node1);
        let root2 = self.find(&node2);

        if root1 == root2 {
            return;
        }

        // if root1 has a greater size (more negative) than root2, it becomes
        // the new root
        if self.nodes[root1] < self.nodes[root2] {
            // add root2's size to root1's
            self.nodes[root1] += self.nodes[root2];
            self.nodes[root2] = root1 as isize;
        } else {
            self.nodes[root2] += self.nodes[root1];
            self.nodes[root1] = root2 as isize;
        }
    }

    /**
    * Finds the root of the requested value's set.
    */
    pub fn find(&mut self, node: &usize) -> usize {
        if node >= &self.nodes.len() {
            panic!("Index out of bounds!");
        }

        // if the requested node is a root, return it.
        if self.nodes[*node] < 0 {
            return *node;
        }

        // otherwise, continue to follow parents until a root is found.
        let mut current: usize = *node;
        let mut data: isize = self.nodes[*node];
        let mut to_compress: Vec<usize> = Vec::new();

        while data >= 0 {
            to_compress.push(current);

            current = data as usize;
            data = self.nodes[current];
        }

        // compress path of each node reached
        for comp_node in to_compress {
            self.nodes[comp_node] = current as isize;
        }

        return current;
    }
}
