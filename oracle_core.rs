use serde::Serialize;
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize)]
pub struct OracleRequest {
    pub req_id: String,
    pub requester: String,
    pub data_source: String,
    pub query: String,
    pub status: OracleStatus,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum OracleStatus {
    Pending,
    Fulfilled,
    Expired,
}

pub struct OracleCore {
    requests: VecDeque<OracleRequest>,
    results: HashMap<String, String>,
}

impl OracleCore {
    pub fn new() -> Self {
        OracleCore {
            requests: VecDeque::new(),
            results: HashMap::new(),
        }
    }

    pub fn request_data(&mut self, req: OracleRequest) {
        self.requests.push_back(req);
    }

    pub fn fulfill_request(&mut self, req_id: String, result: String) -> bool {
        if let Some(req) = self.requests.iter_mut().find(|r| r.req_id == req_id) {
            req.status = OracleStatus::Fulfilled;
            self.results.insert(req_id, result);
            true
        } else {
            false
        }
    }

    pub fn get_result(&self, req_id: &str) -> Option<String> {
        self.results.get(req_id).cloned()
    }
}
