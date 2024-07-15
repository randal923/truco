import os
import fnmatch

def should_ignore(path, ignore_patterns):
    for pattern in ignore_patterns:
        if fnmatch.fnmatch(os.path.basename(path), pattern):
            return True
    return False

def clean_file(file_path):
    if os.path.exists(file_path):
        os.remove(file_path)
        print(f"Cleaned existing file: {file_path}")

def combine_files(input_folder, output_file, ignore_patterns):
    clean_file(output_file)
    
    with open(output_file, 'wb') as outfile:
        for root, dirs, files in os.walk(input_folder):
            # Remove ignored directories
            dirs[:] = [d for d in dirs if not should_ignore(os.path.join(root, d), ignore_patterns)]
            
            for filename in files:
                file_path = os.path.join(root, filename)
                if should_ignore(file_path, ignore_patterns):
                    continue
                
                relative_path = os.path.relpath(file_path, input_folder)
                outfile.write(f"--- Contents of {relative_path} ---\n".encode('utf-8'))
                try:
                    with open(file_path, 'rb') as infile:
                        content = infile.read()
                        try:
                            # Try to decode as UTF-8
                            decoded_content = content.decode('utf-8')
                            outfile.write(content)
                        except UnicodeDecodeError:
                            # If UTF-8 decoding fails, write as binary
                            outfile.write(b"[Binary file or non-UTF-8 encoded, contents not displayed]\n")
                    outfile.write(b"\n\n")
                except Exception as e:
                    outfile.write(f"[Error reading file: {str(e)}]\n\n".encode('utf-8'))

# Example usage
input_folder = "."
output_file = "combined_output.txt"
ignore_patterns = [
    "*.log",                # Ignore all log files
    ".git*",                # Ignore .git folder and .gitignore files
    "*.pyc",                # Ignore Python compiled files
    "node_modules",         # Ignore node_modules folder,
    "*.toml",               # Ignore all TOML files
    "*.lock",               # Ignore all lock files
    "target",               # Ignore target folder
    "compress_files.py",    # Ignore this script
]

combine_files(input_folder, output_file, ignore_patterns)
print(f"All files have been combined into {output_file}")