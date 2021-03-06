use crate::Atom;
use {anyhow::*, hashbrown::HashMap, petgraph::prelude::*, std::borrow::Borrow};

#[derive(Debug, Clone)]
pub struct Node {
    deps: Vec<Atom>,
    graph_index: NodeIndex,
}

pub struct DependencyGraph<T> {
    graph: StableGraph<(Atom, T), ()>,
    indices: HashMap<Atom, Node>,
    sorted: Vec<NodeIndex>,
    changed: bool,
}

impl<T> DependencyGraph<T> {
    pub fn new() -> Self {
        Self {
            graph: StableGraph::new(),
            indices: HashMap::new(),
            sorted: Vec::new(),
            changed: false,
        }
    }

    pub fn insert<I, N, S>(&mut self, value: T, name: N, deps: I) -> Result<Option<T>>
    where
        I: IntoIterator<Item = S>,
        S: Borrow<str>,
        N: Borrow<str>,
    {
        let name = Atom::from(name.borrow());
        let node = self.graph.add_node((name.clone(), value));
        let maybe_old = self.indices.insert(
            name,
            Node {
                deps: deps
                    .into_iter()
                    .map(|s| Atom::from(s.borrow()))
                    .collect::<Vec<_>>(),
                graph_index: node,
            },
        );
        self.changed = true;
        Ok(maybe_old.map(|old| self.graph.remove_node(old.graph_index).unwrap().1))
    }

    pub fn is_dirty(&self) -> bool {
        self.changed
    }

    pub fn update(&mut self) -> Result<bool> {
        if !self.changed {
            return Ok(false);
        }

        let Self { graph, indices, .. } = self;

        graph.clear_edges();
        for node in indices.values() {
            for dep in node.deps.iter().filter_map(|n| indices.get(n)) {
                graph.add_edge(dep.graph_index, node.graph_index, ());
            }
        }

        self.sorted = petgraph::algo::toposort(&self.graph, None).map_err(|cycle| {
            let node = &self.graph[cycle.node_id()].0;
            anyhow!(
                "A cycle was found which includes the node `{}`, \
                but the dependency graph must be acyclic to allow \
                a proper ordering of dependencies!",
                node
            )
        })?;
        self.changed = false;

        Ok(true)
    }

    pub fn sorted(&self) -> impl Iterator<Item = (&str, &T)> {
        assert!(!self.changed);
        self.sorted.iter().copied().map(move |index| {
            let (ref name, ref value) = self.graph[index];
            (name.as_ref(), value)
        })
    }
}
