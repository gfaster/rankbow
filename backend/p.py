#!/usr/bin/env python3

import http.client
import sys
import json
from random import shuffle

def req_any(conn, method, path, *args, **kwargs):
    conn.request(method, path, *args, **kwargs)
    resp = conn.getresponse()
    print(f"{method} request to {path} returned {resp.status}")
    return resp.read()

def req_json(conn, method, path, *args, **kwargs):
    conn.request(method, path, *args, **kwargs)
    resp = conn.getresponse()
    print(f"{method} request to {path} returned {resp.status}")
    return json.loads(resp.read())




def main():
    conn = http.client.HTTPConnection("127.0.0.1:3000")

    id = req_json(conn, "POST", "/create")["id"]
    print(f"Created poll {id=}")

    choices = ["A", "B", "C", "D", "E"]
    for x in range(20):
        shuffle(choices)
        req_any(conn, "POST", f"/poll/{id}/submit", headers={"Content-Type": "application/json"}, body=json.dumps(choices))

    results = req_json(conn, "GET", f"/poll/{id}/results")
    print(results)


if __name__ == "__main__":
    main()


