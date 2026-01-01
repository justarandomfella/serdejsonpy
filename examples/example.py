from typing import Any, List, Dict, Optional
from rich.console import Console
from rich.panel import Panel
from rich.syntax import Syntax

from serdejsonpy import SerdeJSON

console = Console()
sj = SerdeJSON()

# Sample JSON objects
sample_json = {
    "name": "Alice",
    "age": 30,
    "active": True,
    "skills": ["Python", "Rust", None],
    "details": {"height": 165.5, "weight": None},
    "flags": {"flag1": True, "flag2": False, "flag3": "yes", "flag4": 1},
    "empty": None
}

# Helper to print JSON or text in Rich panel
def show_panel(title: str, content: Any, json_format: bool = True):
    if json_format:
        if isinstance(content, (dict, list)):
            content_str = sj.to_json_pretty(content)
        else:
            content_str = str(content)
        syntax = Syntax(content_str, "json", theme="monokai", line_numbers=True)
        panel = Panel(syntax, title=title)
    else:
        panel = Panel(str(content), title=title)
    console.print(panel)


# 1. Basic JSON serialization
show_panel("to_json", sj.to_json(sample_json))
show_panel("to_json_pretty", sj.to_json_pretty(sample_json))

show_panel("to_bytes", sj.to_bytes(sample_json), json_format=False)
show_panel("to_bytes_pretty", sj.to_bytes_pretty(sample_json), json_format=False)

# 2. Deserialization
json_str = sj.to_json(sample_json)
show_panel("from_json", sj.from_json(json_str))
show_panel("from_bytes", sj.from_bytes(sj.to_bytes(sample_json)))

# 3. Type checks
show_panel("is_null 'empty'", sj.is_null(sample_json["empty"]), json_format=False)
show_panel("is_boolean 'active'", sj.is_boolean(sample_json["active"]), json_format=False)
show_panel("is_number 'age'", sj.is_number(sample_json["age"]), json_format=False)
show_panel("is_string 'name'", sj.is_string(sample_json["name"]), json_format=False)
show_panel("is_array 'skills'", sj.is_array(sample_json["skills"]), json_format=False)
show_panel("is_object 'details'", sj.is_object(sample_json["details"]), json_format=False)
show_panel("is_i64 'age'", sj.is_i64(sample_json["age"]), json_format=False)
show_panel("is_u64 'age'", sj.is_u64(sample_json["age"]), json_format=False)
show_panel("is_f64 'height'", sj.is_f64(sample_json["details"]["height"]), json_format=False)

# 4. Conversions
show_panel("as_bool 'active'", sj.as_bool(sample_json["active"]), json_format=False)
show_panel("as_i64 'age'", sj.as_i64(sample_json["age"]), json_format=False)
show_panel("as_u64 'age'", sj.as_u64(sample_json["age"]), json_format=False)
show_panel("as_f64 'height'", sj.as_f64(sample_json["details"]["height"]), json_format=False)
show_panel("as_str 'name'", sj.as_str(sample_json["name"]), json_format=False)

# 5. Merge and update
extra_json = {"new_field": 123, "active": False}
show_panel("merge with extra_json", sj.merge(sample_json, extra_json))

# 6. Accessing and updating
show_panel("get 'name'", sj.get(sample_json, "name"), json_format=False)
updated = sj.set(sample_json, "name", "Bob")
show_panel("set 'name' to 'Bob'", updated)

# 7. Path and pointer utilities
show_panel("get_path ['details','height']", sj.get_path(sample_json, ["details","height"]), json_format=False)
show_panel("pointer '/details/weight'", sj.pointer(sample_json, "/details/weight"), json_format=False)

# 8. Flatten, keys, values, counts
show_panel("flatten", sj.flatten(sample_json))
show_panel("keys", sj.keys(sample_json))
show_panel("values", sj.values(sample_json))
show_panel("count_values", sj.count_values(sample_json))

# 9. Find paths, remove nulls, sort keys, depth
show_panel("find_paths None", sj.find_paths(sample_json, None))
show_panel("remove_nulls", sj.remove_nulls(sample_json))
show_panel("sort_keys", sj.sort_keys(sample_json))
show_panel("depth", sj.depth(sample_json), json_format=False)

# 10. Validation, minify, equals, size
show_panel("validate", sj.validate(json_str), json_format=False)
show_panel("minify", sj.minify(json_str))
show_panel("equals (self vs self)", sj.equals(sample_json, sample_json), json_format=False)
show_panel("size", sj.size(sample_json), json_format=False)
