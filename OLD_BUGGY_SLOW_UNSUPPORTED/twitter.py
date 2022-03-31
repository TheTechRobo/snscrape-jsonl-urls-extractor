import json, sys

print("enter json file:", file=sys.stderr, end="")
with open(input()) as f:
    r = f.readlines()

for i in r:
    try:
        jj = json.loads(i)
    except Exception as ename:
        print(f"invalid JSON, skipping ({ename})", file=sys.stderr)
        continue
    for key in ["url"]:
        if jj.get(key) is None:
            print("\t"+key, file=sys.stderr)
            continue
        print(jj[key])
    print(jj['url'])
    if jj.get('media') is None:
        continue
    for fewh in jj['media']:
        try:
            print(fewh['fullUrl'])
        except Exception:
            for video in fewh['variants']:
                print(video['url'])
    #print(jj['image']) if not (jj.get("image") is None) else print("", end="")
