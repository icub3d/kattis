# A helper to print messages in a consistent style
def "print-info" [message: string] {
    print $"✅ ($message)"
}

# A helper to print error messages
def "print-error" [message: string] {
    print -e $"❌ ERROR: ($message)"
}

# Clear the terminal and scrollback so watch output starts from a clean buffer.
def "reset-terminal" [] {
    print -n "\u{001b}c"
    print -n "\u{001b}[3J\u{001b}[H\u{001b}[2J"
    print -n "\u{001b}[0m"
}

# Kattis runner
export def "kat" [name: string, --quiet (-q)] {
    print $"🐱 Kattis ($name) 🐱"
    
    let input_dir = ("inputs" | path join $name)
    
    if not ($input_dir | path exists) {
        print-error $"Input directory not found: ($input_dir)"
        return
    }
    
    let input_files = (ls $input_dir | where name =~ '\.in$' | get name)
    
    if ($input_files | is-empty) {
        print-error $"No .in files found in ($input_dir)"
        return
    }
    
    print "🔨 Building..."
    let build_result = (cargo build --release -q --bin $name | complete)
    
    if $build_result.exit_code != 0 {
        print-error "Build failed"
        if not ($build_result.stderr | is-empty) {
            print $build_result.stderr
        }
        return
    }
    
    let binary_name = if $nu.os-info.name == "windows" { $"($name).exe" } else { $name }
    let binary_path = ("target" | path join "release" $binary_name)
    
    if not ($binary_path | path exists) {
        print-error $"Binary not found at ($binary_path)"
        return
    }
    
    mut times = []
    
    for $input_file in $input_files {
        let ans_file = ($input_file | str replace '.in' '.ans')
        let test_name = ($input_file | path basename)
        
        if not ($ans_file | path exists) {
            print-error $"Answer file not found: ($ans_file)"
            continue
        }
        
        print $"\n📝 Testing ($test_name)..."
        
        let timing = (timeit { cat $input_file | ^$binary_path | complete })
        let duration = $timing
        
        let output = (cat $input_file | ^$binary_path | complete)
        
        $times = ($times | append $duration)
        
        if $output.exit_code != 0 {
            print-error $"Program failed with exit code ($output.exit_code)"
            if not ($output.stderr | is-empty) {
                print $output.stderr
            }
            continue
        }
        
        let expected = (open --raw $ans_file | str trim)
        let actual = ($output.stdout | str trim)
        
        if $expected == $actual {
            print-info $"Test passed: ($test_name) (($duration))"
        } else {
            print-error $"Test failed: ($test_name) (($duration))"
            if not $quiet {
                print $"Expected:\n($expected)"
                print $"Got:\n($actual)"
            }
        }
    }
    
    if not ($times | is-empty) {
        let avg = ($times | math avg)
        print $"\n⏱️ Average time: ($avg)"
    }
}

export def "kat watch" [name: string] {
    watch-helper $name
    watch --quiet . --glob=**/*.rs {||
      watch-helper $name
    }
}

def watch-helper [name: string] {
    reset-terminal
    try { 
        kat $name
    } catch { |err| 
        print-error $"Compilation failed: ($err.msg)"
        print "🔄 Watching for changes..."
    }
}

# Adds a new kattis problem with the given name
# Usage: kat new carrots
export def "kat new" [name: string ] {
    let path = ("src" | path join "bin" $"($name).rs");

    kat samples $name

    if ($path | path exists) {
        print-error $"Kattis ($name) already exists!"
        return
    }
    let boiler_path = ("src" | path join "bin" "template.rs")
    cat $boiler_path | save --force $path
    print-info $"Created boilerplate for ($name) at '($path)')"
}

###*
# Downloads the puzzle input for a given problem.
# Usage:
# kat samples carrots
###
export def "kat samples" [name: string] {
    let input_dir = ("inputs" | path join $name);
    mkdir $input_dir
    let zip_path  = ($input_dir | path join "samples.zip");


    if ($zip_path | path exists) {
        print-info $"Input for ($name) already exists at '($zip_path)'. Skipping."
        return
    }

    let url = $"https://open.kattis.com/problems/($name)/file/statement/samples.zip";
    print $"🚀 Downloading input for ($name) from ($url)..."

    try {
        let response = http get $url

        # The response from AoC might end in a newline, which we usually want to keep.
        $response | save --force $zip_path
        print-info $"Successfully saved input to '($zip_path)'"
        tar -C $input_dir -xf $zip_path
    } catch { |error|
        print-error "Failed to download input."
        print -e $"Reason: ($error.msg)"
        return
    }
}

# Uploads a solution file to a GitHub Gist using the gh CLI.
# Usage: kat gist carrots
export def "kat gist" [name: string] {
  let file_path = ("src" | path join "bin" $"($name).rs")

    if not ($file_path | path exists) {
        print-error ("solution file not found: " ++ $file_path)
        return
    }

    let gist_desc = $"Kattis ($name)"
    let public_flag = "--public"

    print ("🚀 Uploading " ++ $file_path ++ " to GitHub Gist...")
    let cmd = ["gh" "gist" "create" $file_path "--desc" $gist_desc $public_flag]
    let result = do -i { ^$cmd }
    if ($result | describe) == 'string' {
        print-info "Gist uploaded successfully!"
        $result
    } else if ($result.exit_code? | default 1) == 0 {
        print-info "Gist uploaded successfully!"
        $result.stdout? | default ""
    } else {
        print-error "Failed to upload Gist."
        $result.stderr? | default $result
    }
}

# Generate a YouTube description with timestamps from a stage progress JSON file.
# Usage: kat yt path/to/name.json
export def "kat yt" [
    file: string # The path to the JSON file (e.g., 'name.json')
] {
    let file = ($file | path expand)

    # Validate file exists
    if not ($file | path exists) {
        print-error $"JSON file not found: '($file)'"
        return
    }

    # Derive year and day from the filename (e.g., 'name.json')
    let base = ($file | path basename)
    let name = ($base | str replace ".json" "")

    # --- Find or Create Gist ---
    let filter_str = $"Kattis ($name)"
    let gist_id = (gh gist list --limit 1 --filter $filter_str | split column "\t" | get column1 | first)

    let solution_url = if not ($gist_id | is-empty) {
        $"https://gist.github.com/icub3d/($gist_id)"
    } else {
        # No gist found, so create one and capture the output URL
        kat gist $name
    }

    # Parse JSON
    let data = (open --raw $file | from json)

    # Build problem URL
    let problem_url = $"https://open.kattis.com/problems/($name)"

    # Print header for description
    print "[TODO]"
    print ""
    print $"Problem: ($problem_url)"
    print $"Solution: ($solution_url)"
    print ""

    # Get stage times (fall back to empty if missing)
    let stages = ($data | get stageTimes | default [])

    # Sort stages by startMs to ensure order
    let stages = ($stages | sort-by startMs)

    if ($stages | is-empty) {
        print-info "No 'stageTimes' found in JSON."
        return
    }

    # Print timestamp lines, converting startMs (milliseconds) to a 'M:SS' format.
    for $st in $stages {
        let start_ms = ($st | get startMs | default 0)
        let mins = ($start_ms / 60000 | into int)
        let secs = (($start_ms mod 60000) / 1000 | into int)
        let time_str = (if $secs < 10 { $"($mins):0($secs)" } else { $"($mins):($secs)" })
        let name = ($st | get stageName | default "Unnamed Stage")
        print $"($time_str) ($name)"
    }
}
