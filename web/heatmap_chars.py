from glob import glob
import json


heatmap_chars = dict()


for path in glob('../data/*.json'):
    with open(path, 'r', encoding='utf-8') as f:
        data = json.load(f)

        name = data['name']
        chars = data['chars']
        
        heatmap_chars[name] = chars


heatmap_chars = dict(sorted(heatmap_chars.items()))


with open('./public/heatmap_data.json', 'w+', encoding='utf-8') as f:
    json.dump(heatmap_chars, f, indent='\t', separators=(',', ': '), ensure_ascii=False)
