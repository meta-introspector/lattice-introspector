import json
import subprocess
import os

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

project_root = os.getcwd() # Get the current working directory

def run_grep_command(pattern, exclude_dir):
    command = ["grep", "-r", "-n", "-E", pattern, "--exclude-dir", exclude_dir, "."]
    try:
        result = subprocess.run(command, capture_output=True, text=True, check=True)
        return result.stdout
    except subprocess.CalledProcessError as e:
        if e.returncode == 1: # grep returns 1 if no matches found
            return ""
        else:
            print(f"Error running grep: {e}")
            print(f"Stderr: {e.stderr}")
            return ""

def parse_grep_output(output):
    structured_results = []
    for line in output.splitlines():
        parts = line.split(":", 2) # Split into filepath, line_number, matched_content
        if len(parts) == 3:
            file_path = parts[0].strip()
            line_number = parts[1].strip()
            matched_content = parts[2].strip()
            structured_results.append({
                "file_path": file_path,
                "line_number": line_number,
                "matched_content": matched_content
            })
    return structured_results

def main():
    output_dir = os.path.join(project_root, ".gemini", "prime_resonances")
    os.makedirs(output_dir, exist_ok=True)

    for prime, data in zos_primes_data.items():
        search_terms = data["keywords"] + [str(prime)] + data["text_forms"]
        pattern = "|".join(search_terms)
        
        print(f"Searching for prime {prime} with pattern: {pattern}")
        
        grep_output = run_grep_command(pattern, ".git")
        structured_results = parse_grep_output(grep_output)
        
        output_file_path = os.path.join(output_dir, f"prime_resonance_{prime}.json")
        with open(output_file_path, "w") as f:
            json.dump(structured_results, f, indent=2)
        print(f"Cached results for prime {prime} to {output_file_path}")

if __name__ == "__main__":
    main()
