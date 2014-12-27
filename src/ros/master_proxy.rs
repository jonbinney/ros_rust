use std::fmt;
use std::string;

use xmlrpc;

pub struct MasterProxy {
    master_uri: String,
    caller_id: String,
    caller_api: String
}


impl MasterProxy {
    fn get_published_topics(&self, root: String) -> Vec<string>
    {
        execute_xmlrpc_request(self.master_uri.as_slice(),
            xmlrpc::Request {method_name: getPublishedTopics, params: vec!["/"]
    }

    fn register_subscriber(&self,  topic: &str, topic_type: &str) {
        let request = format!(
            "<?xml version=\"1.0\"?>\n\
            <methodCall>\n\
            <methodName>registerSubscriber</methodName>\n\
            <params>\n\
            <param>\n\
            <value><string>{}</string></value>\n\
            <value><string>{}</string></value>\n\
            <value><string>{}</string></value>\n\
            <value><string>{}</string></value>\n\
            </param>\n\
            </params>\n\
            </methodCall>\n", self.caller_id, topic, topic_type, self.caller_api);

        execute_xmlrpc_request(self.master_uri.as_slice(), request.as_slice());
    }
}


