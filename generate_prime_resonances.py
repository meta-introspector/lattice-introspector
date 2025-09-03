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

for prime, data in zos_primes_data.items():
    search_terms = data["keywords"] + [str(prime)] + data["text_forms"]
    pattern = "|".join(search_terms)
    
    print(f"Searching for prime {prime} with pattern: {pattern}")
    
    # Use a placeholder for default_api.search_file_content
    # In a real execution, this would be replaced by the actual tool call
    # For now, we'll simulate the output or use a mock
    # search_results = default_api.search_file_content(include="**/*", path="/data/data/com.termux/files/home/storage/github/rustc/crates/introspector", pattern=pattern)
    
    # Mock search_results for demonstration
    search_results = {"search_file_content_response": {"output": "Found 1 match for pattern \"test\" in path \"/test\" (filter: \"**/*\"):\n---\nFile: test.txt\nL1: This is a test line.\n---"}}

    if "search_file_content_response" in search_results and "output" in search_results["search_file_content_response"]:
        output_lines = search_results["search_file_content_response"]["output"].splitlines()
        
        structured_results = []
        current_file = None
        for line in output_lines:
            if line.startswith("File:"):
                current_file = line.split("File: ")[1].strip()
            elif line.startswith("L") and current_file:
                parts = line.split(": ", 1)
                if len(parts) > 1:
                    line_info = parts[0]
                    matched_content = parts[1]
                    line_number = line_info.split("L")[1].strip()
                    structured_results.append({
                        "file_path": current_file,
                        "line_number": line_number,
                        "matched_content": matched_content
                    })
        
        output_file_path = f".gemini/prime_resonances/prime_resonance_{prime}.json"
        with open(output_file_path, "w") as f:
            json.dump(structured_results, f, indent=2)
        print(f"Cached results for prime {prime} to {output_file_path}")
    else:
        print(f"No search results or unexpected format for prime {prime}")
