import json, sys

print("Enter Json File:", file=sys.stderr)

with open(input()) as f:
    r = f.readlines()

for i in r:
    try:
        jj = json.loads(i)
    except Exception as ename:
        print(f"invalid JSON, skipping ({ename})", file=sys.stderr)
        continue
    for ids in jj['outlinks']:
        print(ids)
    print(jj['url'])
    print(jj['image']) if not (jj.get("image") is None) else print("", end="")
    if not jj.get("linkPreview") is None:
        print(jj['linkPreview']['image'])
