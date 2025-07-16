export const POS_tags = {
    'lv': {
        "p": { long: 'pronoun', desc: 'Persons or things standing in for nouns (e.g., es, tu – I, you).' },
        "n": { long: 'noun', desc: 'Names of people, places, objects, concepts.' },
        "u": { long: 'NULL/unknown', desc: 'No clearly defined single POS; used for entries without a specific POS.' },
        "v": { long: 'verb', desc: 'Action or state words (infinitive, finite, participle forms).' },
        "x": { long: 'other', desc: 'Residual categories: conjunctions, particles, interjections, residual extras.' },
        "a": { long: 'adjective', desc: 'Describing words (qualitative or relative: large, red, similar).' },
        "r": { long: 'adverb', desc: 'Modifiers of verbs, adjectives, or other adverbs (e.g., ātri – quickly).' }
    },
    'en': {
        "n": { long: 'noun', desc: 'Entities: persons, places, things, or concepts.' },
        "v": { long: 'verb', desc: 'Denotes actions, occurrences, or states.' },
        "a": { long: 'adjective', desc: 'Descriptive words modifying nouns.' },
        "s": { long: 'adj satellite', desc: 'A type of adjective closely linked to a head adjective (e.g., "afraid" with "fearful").' },
        "r": { long: 'adverb', desc: 'Modifies verbs, adjectives, or other adverbs.' }
    }
}
