import json

with open(input("Enter Json File:")) as f:
    r = f.readlines()

for i in r:
    try:
        jj = json.loads(i)
    except Exception as ename:
        print(f"invalid JSON, skipping ({ename})")
        continue
    for ids in jj['outlinks']:
        print(ids)
    print(jj['url'])
    print(jj['image']) if not (jj.get("image") is None) else print("", end="")
