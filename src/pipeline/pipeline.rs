use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::error::Result;
use crate::stream::{Stream, StreamId};

type Graph = HashMap<StreamId, Vec<StreamId>>;

pub struct Pipeline {
    queue: Vec<StreamId>,
    streams: HashMap<StreamId, Box<dyn Stream>>,
    graph: Graph,
}

impl Pipeline {
    fn new() -> Self {
        Self {
            queue: Vec::new(),
            streams: HashMap::new(),
            graph: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let stream_id = match self.queue.pop() {
                Some(stream_id) => stream_id,
                None => break,
            };

            let stream = match self.streams.get_mut(&stream_id) {
                Some(stream) => stream,
                None => panic!("Stream not found"),
            };

            let point = match stream.read()? {
                Some(point) => Rc::new(point),
                None => continue,
            };

            self.queue.push(stream_id);

            for upstream_id in self
                .graph
                .get(&stream_id)
                .unwrap_or(&Vec::new())
                .iter()
                .cloned()
            {
                let upstream = match self.streams.get_mut(&upstream_id) {
                    Some(upstream) => upstream,
                    None => panic!("Upstream not found"),
                };
                upstream.write(stream_id, point.clone());
                self.queue.push(upstream_id);
            }
        }
        Ok(())
    }
}

pub struct PipelineBuilder {
    pipeline: Pipeline,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            pipeline: Pipeline::new(),
        }
    }

    pub fn add_stream(&mut self, downstreams: Vec<StreamId>, stream: Box<dyn Stream>) -> &mut Self {
        let stream_id = *stream.id();
        self.pipeline.streams.insert(stream_id, stream);
        for downstream_id in downstreams.iter().cloned() {
            self.pipeline
                .graph
                .entry(downstream_id.into())
                .or_insert(Vec::new())
                .push(stream_id);
        }
        self
    }

    pub fn build(mut self) -> Pipeline {
        // TODO: ensure graph is DAG with single source
        self.pipeline
            .queue
            .push(find_source_stream(&self.pipeline.graph));
        self.pipeline
    }
}

fn find_source_stream(graph: &Graph) -> StreamId {
    let mut nodes: HashSet<&StreamId> = graph.keys().collect();
    for dst in graph.values().flatten() {
        nodes.remove(&dst);
    }

    if nodes.len() == 0 {
        panic!("Stream graph doesn't have a source");
    }
    if nodes.len() > 1 {
        panic!("Stream graph has multiple sources instead of 1");
    }
    **nodes.iter().next().unwrap()
}
