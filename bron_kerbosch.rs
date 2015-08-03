/// Implementation according to "Algorithm 457: Finding All Cliques of an Undirected Graph"
/// by Bronand Kerbosch; http://doi.acm.org/10.1145/362342.362367
///
/// connected is a symmetrical bolean matrix, N the number of nodes in the graph,
/// values of the diagonal should be true.


pub fn main() {
    graph =
    bk = BronKerbosch::new();
}

struct Graph {
    size: i32,
    map: HashMap<i32, Vec<i32>>
}

impl Graph {
    pub fn new(size: i32) -> Graph {
        Graph {
            size: size,
            map: HashMap::new()
        }
    }

    pub fn add_edge(&mut self, from: i32, to: i32) {
        self.insert(from, to);
    }

    pub fn connected(&self, from, to) {

    }
}





struct BronKerbosch {
    compsub: Vec<i32>,
    connected: Graph,
    c: i32
}

impl BronKerbosch {
    fn new() -> BronKerbosch {
        BronKerbosch {
            compsub: Vec<i32>,
            c: 0
        }
    }

    fn compute(&self, N: i32) {
        let ALL = (1..N+1).collect();
        extend_version_2(ALL, 0, N);
    }




    fn output_maximal_complete_subgraphs_2(N: i32) {
        let ALL: i32[];
        let compsub: [[i32; N]; N]; // compsub_1 : i32[], compsub_2 : i32[] ... compsub_N: i32[]
        let c: i32;
    }


    fn extend_version_2(old: i32[], ne: i32, ce: i32) {

        let new: [[i32; N]; ce]; // new_1 : i32[], new_2 : i32[] ... new_ce: i32[]
        let nod: i32 = 0;
        let fixp: i32;
        let newne: i32;
        let newce: i32;
        let j: i32;
        let count: i32;
        let pos: i32;
        let p: i32;
        let s : i32;
        let sel: i32;

        let minnod: i32 = ce;

        // determine each counter value and look for minimum
        determine_counter();

        // backtrackcycle
        backtrack();
    }

    fn determine_counter(ce: i32, old: Vec<i32>, ne: i32) {
        for i in (1..ce+1) {
            p = old[i];
            j = ne;

            // count disconnections

            count, pos = count_disconnections(j, ce, old);
            fixp, minnod, s, nod = test_new_minimum(count, minnod, p, i, ne, pos);

            if minnod == 0 {
                break;
            }
        }
    }

    // DONE
    fn count_disconnections(&self, j: i32, ce: i32, old: Vec<i32>) -> (i32, i32) {
        let mut count = 0;

        for jj in (j+1..ce+1) {
            if self.graph.connected(p, old[j]) {
                count += 1;
                pos = j;
            }

            if !(count < minnod) {
                break;
            }
        }

        (count, pos)
    }

    // DONE
    fn test_new_minimum(count: i32, minnod: i32, p: i32, i: i32, ne: i32, pos: i32) -> (i32, i32, i32, i32){
        // test new minimum
        if count < minnod {
            fixp = p;
            minnod = count;
            if i <= ne {
                s = pos;
            } else {
                s = i;
                nod = 1;
            }
        }
        (fixp, minnod, s, nod)
    }

    fn backtrack() {
        for nod = (minnod + nod .. 0).step_by(-1) {
            p = old[s];
            old[s] = old[ne + 1];
            sel = p;
            old[ne + 1] = p;

            newne = fill_new_set_not(ne, old, new);
            fill_new_set_cand(newne, ne, ce, sel, old, new);
            add_to_compsub();
        }

    }

    fn fill_new_set_not(&self, ne: i32, old: Vec<i32>, new: Vec<i32>, sel: i32) -> i32 {
        // fill new set "not"
        let mut newne = 0;

        for i in (1..ne+1) {
            if self.graph.connected(sel, old[i]) {
                newne = newne + 1;
                new[newne] = old[i];
            }
        }
    }

    fn fill_new_set_cand(newne: i32, ne: i32, ce: i32, sel: i32, old: Vec<i32>, new: Vec<i32>) {
        // fill new set "cand"
        newce = newne;
        i = ne + 1;
        for i in (ne+1..ce+1){
            if self.graph.connected(sel, old[i]) {
                newce = newce + 1;
                new[newce] = old[i]
            }
        }
    }

    fn add_to_compsub(&mut self, sel, new, newce, newne, ne) {
        // add to "compsub"
        self.c += 1;
        self.compsub[self.c] = sel;

        if newce == 0 {
            print!("clique = ")
            for loc in (1..self.c) {
                print!(" {} ", self.compsub[loc])
            }
        } else {
            if newne < newce {
                extend_version_2(new, newne, newce);
            }
            // remove from "compsub"
            self.c -= 1;
            // add to "not"
            ne += 1;

            if nod > i {
                // select a candidate disconnected to the fixed point
                s = ne;
                s += 1;
                if self.graph.connected(fixp, old[s]) {
                    goto LOOK
                }
            }
        }
    }
}

