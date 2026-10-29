use serde_json::{Value, json};

pub fn candidate() -> Value {
    let evidence = json!({"type":"array","minItems":1,"maxItems":4,"items":{
        "type":"object","properties":{"file":{"type":"string","maxLength":240},
        "line":{"type":"integer","minimum":1},"exact":{"type":"string","maxLength":1024}},
        "required":["file","line","exact"],"additionalProperties":false}});
    let id = json!({"type":"string","minLength":1,"maxLength":64,"pattern":"^[A-Za-z0-9_]+$"});
    json!({"type":"object","properties":{
        "version":{"type":"integer","const":1},
        "sources":{"type":"array","maxItems":32,"items":{"type":"string","maxLength":240}},
        "nodes":{"type":"array","maxItems":128,"items":{"type":"object","properties":{
            "id":id,"kind":{"type":"string","enum":["variable","parameter","operation","loss","constraint","state","input","preprocess","model","evaluation","output"]},
            "label":{"type":"string","minLength":1,"maxLength":160},"evidence":evidence,
            "interpretation":{"type":["object","null"],"properties":{
                "expression":{"type":"string","minLength":1,"maxLength":512},
                "confidence":{"type":"number","minimum":0,"maximum":1}},
                "required":["expression","confidence"],"additionalProperties":false},
            "metadata":{"type":["object","null"],"properties":{
                "path_reference":{"type":["string","null"],"maxLength":240},
                "format":{"type":["string","null"],"maxLength":256},
                "shape":{"type":["string","null"],"maxLength":256},
                "schema":{"type":["string","null"],"maxLength":256},
                "units":{"type":["string","null"],"maxLength":256}},"additionalProperties":false}},
            "required":["id","kind","label","evidence"],"additionalProperties":false}},
        "edges":{"type":"array","maxItems":256,"items":{"type":"object","properties":{
            "from":id,"to":id,"relation":{"type":"string","enum":["uses","feeds","updates","constrains","evaluates","produces"]},
            "evidence":evidence},"required":["from","to","relation","evidence"],"additionalProperties":false}}
    },"required":["version","sources","nodes","edges"],"additionalProperties":false})
}
