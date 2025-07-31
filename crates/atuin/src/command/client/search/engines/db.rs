use async_trait::async_trait;
use atuin_client::{
    database::Database, database::OptFilters, history::History, settings::SearchMode,
};
use eyre::Result;
use norm::Metric;
use norm::fzf::{FzfParser, FzfV2};
use std::ops::Range;

use super::{SearchEngine, SearchState};

pub struct Search(pub SearchMode);

#[async_trait]
impl SearchEngine for Search {
    async fn full_query(
        &mut self,
        state: &SearchState,
        db: &mut dyn Database,
    ) -> Result<Vec<History>> {
        Ok(db
            .search(
                self.0,
                state.filter_mode,
                &state.context,
                state.input.as_str(),
                OptFilters {
                    limit: Some(200),
                    ..Default::default()
                },
            )
            .await
            // ignore errors as it may be caused by incomplete regex
            .map_or(Vec::new(), |r| r.into_iter().collect()))
    }
    fn get_highlight_indices(&self, command: &str, search_input: &str) -> Vec<usize> {
        let mut fzf = FzfV2::new();
        let mut parser = FzfParser::new();
        let query = parser.parse(search_input);
        let mut ranges: Vec<Range<usize>> = Vec::new();
        let _ = fzf.distance_and_ranges(query, command, &mut ranges);

        // convert ranges to all indices
        ranges.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod search_engine_tests {
    use super::*;

    #[test]
    fn test_fuzzy() {
        let search = Search(SearchMode::Fuzzy);
        let vec = search.get_highlight_indices("podman volume ls", "podvl");
        assert_eq!(vec, vec![0, 1, 2, 7, 14]);
    }
}
