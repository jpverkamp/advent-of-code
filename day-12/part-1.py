#!/usr/bin/env python3

import json
import sys

def js_sum(js):
    if isinstance(js, dict):
        return sum(map(js_sum, js.values()))
    elif isinstance(js, list):
        return sum(map(js_sum, js))
    elif isinstance(js, int):
        return js
    else:
        return 0

print(js_sum(json.load(sys.stdin)))
