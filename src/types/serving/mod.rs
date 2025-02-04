use super::*;
use crate::internals::priority_queue::PriorityQueue;

pub trait AnnoyIndexSearchApi {
    fn get_item_vector(&self, item_index: u64) -> Vec<f32>;
    fn get_nearest(
        &self,
        query_vector: &[f32],
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> AnnoyIndexSearchResult;
    fn get_nearest_to_item(
        &self,
        item_index: u64,
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> AnnoyIndexSearchResult;
}

impl AnnoyIndexSearchApi for AnnoyIndex {
    fn get_item_vector(&self, item_index: u64) -> Vec<f32> {
        let node_offset = item_index as usize * self.node_size;
        let slice = self.get_node_slice_with_offset(node_offset);
        slice.to_vec()
    }

    fn get_nearest(
        &self,
        query_vector: &[f32],
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> AnnoyIndexSearchResult {
        let result_capacity = n_results.min(self.size).max(1);
        let search_k_fixed = if search_k > 0 {
            search_k as usize
        } else {
            result_capacity * self.roots.len()
        };

        let mut pq = PriorityQueue::with_capacity(result_capacity, false);
        for &id in self.roots.iter() {
            pq.push(id as i32, f32::MAX);
        }

        let mut nearest_neighbors = Vec::with_capacity(search_k_fixed);
        while pq.len() > 0 && nearest_neighbors.len() < search_k_fixed {
            if let Some((top_node_id_i32, top_node_margin)) = pq.pop() {
                let top_node_id = top_node_id_i32 as usize;
                let top_node = self.get_node_from_id(top_node_id);
                let top_node_header = top_node.header;
                let top_node_offset = top_node.offset;
                let n_descendants = top_node_header.get_n_descendant();
                if n_descendants == 1 && top_node_id < self.size {
                    nearest_neighbors.push(top_node_id_i32);
                } else if n_descendants <= self.max_descendants {
                    let children_id_slice =
                        self.get_descendant_id_slice(top_node_offset, n_descendants as usize);
                    nearest_neighbors.extend_from_slice(children_id_slice);
                } else {
                    let v = self.get_node_slice_with_offset(top_node_offset);
                    let margin = self.get_margin(v, query_vector, top_node_offset);
                    let [child_0, child_1] = top_node_header.get_children_id_slice();
                    // NOTE: Hamming has different logic to calculate margin
                    pq.push(child_1, top_node_margin.min(margin));
                    pq.push(child_0, top_node_margin.min(-margin));
                }
            }
        }
        nearest_neighbors.sort_unstable();
        let mut sorted_nns = PriorityQueue::with_capacity(nearest_neighbors.len(), true);
        let mut nn_id_last = -1;
        for nn_id in nearest_neighbors {
            if nn_id == nn_id_last {
                continue;
            }
            nn_id_last = nn_id;
            let node = self.get_node_from_id(nn_id as usize);
            let n_descendants = node.header.get_n_descendant();
            if n_descendants != 1 {
                continue;
            }

            let s = self.get_node_slice_with_offset(nn_id as usize * self.node_size);
            sorted_nns.push(nn_id, self.get_distance_no_norm(s, query_vector));
        }

        let final_result_capcity = n_results.min(sorted_nns.len());
        let mut id_list = Vec::with_capacity(final_result_capcity);
        let mut distance_list = Vec::with_capacity(if should_include_distance {
            final_result_capcity
        } else {
            0
        });
        for _i in 0..final_result_capcity {
            let nn = &sorted_nns.pop().unwrap();
            id_list.push(nn.0 as u64);
            if should_include_distance {
                distance_list.push(self.normalized_distance(nn.1));
            }
        }
        AnnoyIndexSearchResult {
            count: final_result_capcity,
            is_distance_included: should_include_distance,
            id_list,
            distance_list,
        }
    }

    fn get_nearest_to_item(
        &self,
        item_index: u64,
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> AnnoyIndexSearchResult {
        let item_vector = self.get_item_vector(item_index);
        self.get_nearest(
            item_vector.as_slice(),
            n_results,
            search_k,
            should_include_distance,
        )
    }
}
