use zama_challenge::thread::ThreadPool;
use crate::executable_graph::{Operation, ExecutableGraph};
use std::time::Duration;

mod executable_graph;


fn main(){
    let thread_pool = ThreadPool::<u32>::new(6);
//    let mut g = predefined_graph(thread_pool);
    let mut g = ExecutableGraph::generate_random(
        100,
        3,
        vec![vec![3,4], vec![4,5,1,1], vec![3,7,12,20,55]],
        vec![
            Operation{f:|x| x.iter().sum(), name: String::from("sum")},
            Operation{f:|x| x[0]+1, name: String::from("incr")},
            Operation{
                f:|x| {
                    let n = x.len();
                    std::thread::sleep(Duration::new(n as u64, 0));
                    n as u32
                },
                name: String::from("sleep")
            },
        ],
        Some(vec![0.1,0.6,0.3]),
        thread_pool
    );

    g.show_graph();
    g.start();
    g.show_graph();

}

pub fn predefined_graph(thread_pool: ThreadPool<u32>) -> ExecutableGraph<u32>{
    let mut eg = ExecutableGraph::new(thread_pool);
    let init_sum = eg.add_initial_node(
        Operation{f:|x| x.iter().sum(),
            name: String::from("init_sum")},
        vec![5,2,3],
    );
    let init_sum2 = eg.add_initial_node(
        Operation{f:|x| x.iter().sum(),
            name:String::from("init_sum2")},
        vec![3,3,3,4,4],
    );
    let init_prod = eg.add_initial_node(
        Operation{f: |x| x.iter().product(),
            name: String::from("init_prod")},
        vec![2,3,5,4],
    );
    let sleep = eg.add_node(
        Operation{ f:|x| {
            let n = x.len();
            std::thread::sleep(Duration::new((n*4) as u64, 0));
            n as u32
        },
            name: String::from("sleep")
        },
        vec![init_sum, init_prod],
    );
    let sqsum = eg.add_node(
        Operation{f: |x| x.iter().map(|x| x.pow(2)).sum(),
            name: String::from("sqsum")},
        vec![init_sum, sleep, init_prod],
    );

    let sum2 = eg.add_node(
        Operation{f: |x| x.iter().sum(),
            name: String::from("sum2")},
        vec![init_prod, init_sum2],
    );

    let _sum3 = eg.add_node(
        Operation{f:|x| x.iter().sum(),
            name: String::from("sum3")},
        vec![sum2],
    );

    let _sum4 = eg.add_node(
        Operation{f: |x| x.iter().sum(),
            name:String::from("sum4")},
        vec![sum2, sqsum],
    );
    eg
}