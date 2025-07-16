import xml.etree.ElementTree as ET
import json

t = -1
def PrintTagsRecursive(e: ET.Element):
    global t
    t += 1
    i = 0
    for child in e:
        print('\t'.join(['\t' * t, child.tag, str(child.text or '-'), str(child.attrib)]))
        if len(child): PrintTagsRecursive(child)
        if i > 5: break
        i += 1
    t -= 1

def PrintTree(e: ET.Element):
    print('\t'.join([e.tag, str(e.text or '-'), str(e.attrib) ]))
    PrintTagsRecursive(e)

# Function to parse WordNet XML and extract required information
def parse_wordnet(xml_file_path, json_file_path, id_prefix=''):
    tree = ET.parse(xml_file_path)
    root = tree.getroot()

    Lexicon = root.find(".//Lexicon")

    pos_tags = set()
    prefix_len = len(id_prefix)

    if True:
        synsets = {}
        for i, Synset in enumerate(root.find(".//Lexicon").findall('Synset')):
            if i % 10000 == 0:
                print(f"Processed {i} synsets...")
            # PrintTree(Synset)
            # break
            id = Synset.get('id')[prefix_len:]
            # pos = Synset.get('partOfSpeech') or ''
            gloss = list(filter(lambda x: x.count(' ') > 0, map(lambda x: (x.text or '').strip(), Synset.findall('Definition') or [])))
            examples = list(filter(lambda x: x.count(' ') > 0, map(lambda x: (x.text or '').strip(), Synset.findall('Example') or [])))
            if len(examples) > 2: examples = examples[:2]

            # rels = list(map(lambda x: x.get('target'), filter(lambda x: x.get('relType') in ['synonym', 'antonym'], Synset.findall('SynsetRelation') or [])))
            synonym = list(map(lambda x: x.get('target')[prefix_len:], filter(lambda x: x.get('relType') == 'similar', Synset.findall('SynsetRelation') or [])))
            antonym = list(map(lambda x: x.get('target')[prefix_len:], filter(lambda x: x.get('relType') == 'antonym', Synset.findall('SynsetRelation') or [])))

            synsets[id] = {'defs':gloss, 'ex':examples, 'syns':synonym, 'ants':antonym}
            # break
        # print(synsets)
        # print(len(synsets))
        # return

    words = {}
    j = 0
    for i, LexicalEntry in enumerate(Lexicon.findall('LexicalEntry')):
        if i % 10000 == 0:
            print(f"Processed {i} lexical entries...")
        # PrintTree(LexicalEntry)
        # break
        word = (LexicalEntry.find("Lemma").get("writtenForm") or '').strip()
        if word.count(' ') > 2: continue
        pos = LexicalEntry.find("Lemma").get("partOfSpeech") or ''
        pos_tags.add(pos)

        senses = []
        for sense in LexicalEntry.findall('Sense'):
            synset_id = sense.get('synset') or ''
            synset_id = synset_id[prefix_len:]
            senses.append(synset_id)

        if word and len(senses):
            if word not in words:
                words[word] = {pos:senses}
            else:
                j += 1
                # if j < 50: print(word)
                words[word][pos] = senses

        # break
    # print(words)
    # print(len(words), j)

    if True:
        with open(json_file_path, 'w', encoding='utf-8') as json_file:
            json.dump({'pos_tags':list(pos_tags), 'synsets':synsets, 'words':words}, json_file, indent=2, ensure_ascii=False)
        print(f"JSON file written successfully: {json_file_path}")


# xml_file_path = 'wordnets/en_wordnet_lmf_2024.xml'
# json_file_path = 'src-tauri/resources/en_wordnet_lmf_2024.json'
# parse_wordnet(xml_file_path, json_file_path)

parse_wordnet('wordnets/en_wordnet_lmf_2024.xml', 'src-tauri/resources/en_wordnet_lmf_2024.json', 'oewn-')
parse_wordnet('wordnets/lv_wordnet_lmf_2025.xml', 'src-tauri/resources/lv_wordnet_lmf_2025.json', 'wordnet_lv-tezaurs_2025_2-')
