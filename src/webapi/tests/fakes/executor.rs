use super::super::super::{entities::executor, connectors};

pub struct SendedAsyncCommandCollection {
    items: Vec<executor::SendedAsyncCommand>,
}

impl SendedAsyncCommandCollection {
    pub fn new() -> SendedAsyncCommandCollection {
        let items = vec![];
        SendedAsyncCommandCollection { items: items }
    }

    pub async fn get(&self, _ids: Option<Vec<String>>) -> connectors::Result<Vec<executor::SendedAsyncCommand>> {
        Ok(self.items.clone())
    }
}

pub struct ReceivedAsyncCommandCollection {
    items: Vec<executor::ReceivedAsyncCommand>,
}

impl ReceivedAsyncCommandCollection {
    pub fn new() -> ReceivedAsyncCommandCollection {
        let items = vec![];
        ReceivedAsyncCommandCollection { items: items }
    }

    pub async fn get(&self, _ids: Option<Vec<String>>) -> connectors::Result<Vec<executor::ReceivedAsyncCommand>> {
        Ok(self.items.clone())
    }
}
