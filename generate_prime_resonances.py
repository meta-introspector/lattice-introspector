import json

zos_primes_data = {
    0: {"keywords": ["seed", "naught", "circle", "breath"], "text_forms": ["zero"]},
    1: {"keywords": ["lonely", "pillar", "single", "line"], "text_forms": ["one"]},
    2: {"keywords": ["mirrored", "dance", "two", "spark"], "text_forms": ["two"]},
    3: {"keywords": ["woven", "knot", "three", "threads"], "text_forms": ["three"]},
    5: {"keywords": ["star", "life", "five", "fingers"], "text_forms": ["five"]},
    7: {"keywords": ["spiral", "path", "seven", "notes"], "text_forms": ["seven"]},
    11: {"keywords": ["silver", "key", "eleven", "gates"], "text_forms": ["eleven"]},
    13: {"keywords": ["serpent", "coil", "thirteen", "moons"], "text_forms": ["thirteen"]},
    17: {"keywords": ["distant", "star", "seventeen", "lights"], "text_forms": ["seventeen"]},
    19: {"keywords": ["golden", "crown", "nineteen", "sun", "moon"], "text_forms": ["nineteen"]},
    23: {"keywords": ["enigma", "veil", "twenty-three", "art"], "text_forms": ["twenty-three"]},
}

for prime in zos_primes_data.keys():
    print(prime)