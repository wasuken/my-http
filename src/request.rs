use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: String,
}

pub fn parse_request(raw_request: &str) -> Request {
    let mut request = Request {
	method: HttpMethod::GET,
	path: String::new(),
	headers: HashMap::new(),
	query_params: HashMap::new(),
	body: String::new(),
    };

    let lines: Vec<&str> = raw_request.lines().collect();

    // リクエストラインの解析
    let request_line = lines[0];
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() >= 3 {
	request.method = match parts[0] {
	    "GET" => HttpMethod::GET,
	    "POST" => HttpMethod::POST,
	    "PUT" => HttpMethod::PUT,
	    "DELETE" => HttpMethod::DELETE,
	    _ => HttpMethod::GET, // デフォルト値としてGETメソッドを設定
	};
	request.path = parts[1].to_string();
    }

    // ヘッダーの解析
    let mut i = 1;
    while i < lines.len() {
	let line = lines[i];
	if line.is_empty() {
	    break; // 空行に到達したらヘッダーの解析を終了
	}
	let header_parts: Vec<&str> = line.splitn(2, ':').map(|part| part.trim()).collect();
	if header_parts.len() == 2 {
	    request
		.headers
		.insert(header_parts[0].to_string(), header_parts[1].to_string());
	}
	i += 1;
    }

    // ボディの解析
    for j in i..lines.len() {
	let line = lines[j];
	request.body.push_str(line);
	request.body.push('\n');
    }

    request
}

#[cfg(test)]
mod tests {
    use super::*; // srcの全ての公開内容をインポートする

    #[test]
    fn test_parse_request() {
	let raw_request = "GET /path HTTP/1.1\nHost: example.com\nContent-Type: application/json\n\n{\"key\": \"value\"}";
	let expected_request = Request {
	    method: HttpMethod::GET,
	    path: String::from("/path"),
	    headers: {
		let mut headers = HashMap::new();
		headers.insert(String::from("Host"), String::from("example.com"));
		headers.insert(
		    String::from("Content-Type"),
		    String::from("application/json"),
		);
		headers
	    },
	    query_params: HashMap::new(),
	    body: String::from("\n{\"key\": \"value\"}\n"),
	};

	let request = parse_request(raw_request);
	assert_eq!(request.method, expected_request.method);
	assert_eq!(request.path, expected_request.path);
	assert_eq!(request.headers, expected_request.headers);
	assert_eq!(request.query_params, expected_request.query_params);
	assert_eq!(request.body, expected_request.body);
    }
}
