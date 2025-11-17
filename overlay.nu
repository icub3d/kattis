export def "kat open" [name: string] {
  ^firefox-developer-edition --new-window $"https://open.kattis.com/problems/($name)"
}


# Kattis runner
export def "kat" [name: string, --quiet (-q), --relative-error (-r): number] {
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

    for $input_file in $input_files {
        let ans_file = ($input_file | str replace '.in' '.ans')
        let test_name = ($input_file | path basename)

        if not ($ans_file | path exists) {
            print-error $"Answer file not found: ($ans_file)"
            continue
        }

        let timing = (timeit { cat $input_file | ^$binary_path | complete })
        let duration = $timing

        let output = (cat $input_file | ^$binary_path | complete)

        if $output.exit_code != 0 {
            print-error $"Program failed with exit code ($output.exit_code)"
            if not ($output.stderr | is-empty) {
                print $output.stderr
            }
            continue
        }

        let expected = (open --raw $ans_file | str trim)
        let actual = ($output.stdout | str trim)

        if $relative_error != null {
            try {
                let exp = ($expected | into float)
                let act = ($actual | into float)
                let abs_diff = (($act - $exp) | math abs)
                let denom = ($exp | math abs)
                let rel = if $denom != 0 { $abs_diff / $denom } else { $abs_diff }
                if $rel <= $relative_error {
print $"📝 ($test_name) ✅ \(($duration))"
                } else {
print $"📝 ($test_name) ❌ \(($duration))"
                    if not $quiet {
                        print $"Expected:\n($expected)"
                        print $"Got:\n($actual)"
                        print $"Variance: ($rel)"
                    }
                }
            } catch { |err|
                print-error $"Failed to compare as floats: ($err.msg)"
            }
        } else {
            if $expected == $actual {
    print $"📝 ($test_name) ✅ \(($duration))"
            } else {
    print $"📝 ($test_name) ❌ \(($duration))"
                if not $quiet {
                    print $"Expected:\n($expected)"
                    print $"Got:\n($actual)"
                }
            }
        }
    }
}

export def "kat show" [name: string] {
  let url = $"https://open.kattis.com/problems/($name)"
  http get $url | 
    pup .problembody --charset UTF-8 | 
    w3m -T text/html -dump |
    lines |
    str replace -m -r '^\s+' '' |
    each {|l| 
      if ($l | str starts-with '/') { 
        print (img $"https://open.kattis.com($l)") 
      } else { 
        print $l 
      } 
    } | ignore
}

export def "kat watch" [name: string, --relative-error (-r): number] {
    watch-helper $name --relative-error=$relative_error
    watch --quiet . --glob=**/*.rs {||
      watch-helper $name --relative-error=$relative_error
    }
}

def watch-helper [name: string, --relative-error (-r): number] {
    reset-terminal
    try { 
        kat $name --relative-error=$relative_error
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
        cd $input_dir
        unzip samples.zip
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

export def "kat submit" [name: string] {
  let config_path = ".kattis.ini"
  if not ($config_path | path exists) {
    print-error "no .kattis.ini found. Go to https://open.kattis.com/info/submit"
    return
  }

  let config = (open $config_path | parse ini)
  let language = 'Rust'
  let file_path = ("src" | path join "bin" $"($name).rs")

  if not ($file_path | path exists) {
      print-error $"Solution file not found: ($file_path)"
      return
  }

  # 1. Login and get cookies
  print -n "Status: Logging in..."
  let login_response = http post -f --content-type "application/x-www-form-urlencoded" --headers ['User-Agent', 'kattis-cli-submit'] ($config | get kattis.loginurl) {
    user: ($config | get user.username),
    token: ($config | get user.token),
    script: "true"
  }

  if $login_response.status != 200 {
      print-error "Login failed."
      return
  }

  let cookies = ($login_response.headers.response
    | where name == "set-cookie"
    | get value
    | each { |s| $s | split row ";" | first }
    | str join "; ")

  # 2. Prepare submission data and submit
  # Note: Nushell's http post doesn't handle complex multipart forms well, so we use curl
  print -n $"\r(ansi erase_line)Status: Submitting..."
  
  let submit_url = ($config | get kattis.submissionurl)
  let file_basename = ($file_path | path basename)
  
  let submit_output = (do -i {
    ^curl -sS -H "User-Agent: kattis-cli-submit" -H $"Cookie: ($cookies)" -F "submit=true" -F "submit_ctr=2" -F $"language=($language)" -F $"problem=($name)" -F "script=true" -F $"mainclass=($file_basename)" -F $"sub_file[]=@($file_path)" $submit_url
  } | complete)

  if $submit_output.exit_code != 0 {
      print-error "Submission failed."
      print $submit_output.stderr
      return
  }

  let response_text = $submit_output.stdout
    # Parse the submission id robustly (HTML or plain text)
  let submission_id = do {
    let found_lines = ($response_text | lines | find -r '^\s*Submission ID:\s*')
    let by_line = if ($found_lines | is-empty) { "" } else { 
      $found_lines | first | str replace -r '^\s*Submission ID:\s*' "" | str trim 
    }
    if not ($by_line | is-empty) { $by_line } else {
      let parsed = ($response_text | parse -r 'Submission ID:\s*(?<id>\d+)')
      if ($parsed | is-empty) { "" } else { $parsed | get id | first }
    }
  }

  if ($submission_id | is-empty) {
      print "" # newline
      print-error "Could not get submission ID from response:"
      print $response_text
      return
  }

  let submission_url = $"(($config | get kattis.submissionsurl))/($submission_id)"

  # 4. Check status
  let get_status_text = {|id|
    match $id {
      0 | 1 => 'New',
      2 => 'Waiting for compile',
      3 => 'Compiling',
      4 => 'Waiting for run',
      5 => 'Running',
      6 => 'Judge Error',
      8 => 'Compile Error',
      9 => 'Run Time Error',
      10 => 'Memory Limit Exceeded',
      11 => 'Output Limit Exceeded',
      12 => 'Time Limit Exceeded',
      13 => 'Illegal Function',
      14 => 'Wrong Answer',
      16 => 'Accepted',
      _ => $"Unknown status ($id)"
    }
  }

  while true {
    sleep 1sec
    let status_url = $"($submission_url)?json"
    let status_data = (http get --headers [
        "User-Agent", "kattis-cli-submit",
        "Cookie", $cookies
      ] $status_url)

    let status_id = ($status_data.status_id | into int)
    let status_text = (do $get_status_text $status_id)

    if $status_id < 5 { # Not running yet
        print -n $"\r(ansi erase_line)Status: ($status_text)"
    } else {
        let testcases_done = $status_data.testcase_index
        let row_html = ($status_data.row_html | default "")
        # Try to count testcases; fall back gracefully if tools are unavailable
        let testcases_total = (try { 
          let count_str = ($row_html | ^pup 'i json{}' | from json | length)
          $count_str - 1
        } catch { 0 })
        if $testcases_total > 0 {
          print -n $"\r(ansi erase_line)Status: ($status_text) [($testcases_done)/($testcases_total)]"
        } else {
          print -n $"\r(ansi erase_line)Status: ($status_text) [($testcases_done)]"
        }
    }

    if $status_id > 5 { # Finished
        print "" # Newline after final status
        if $status_id == 16 { # Accepted
            print $"✅ ($status_text)"
        } else {
            print $"❌ ($status_text)"
        }
        let feedback_html = (try { $status_data.feedback_html } catch { "" })
        if not ($feedback_html | is-empty) {
            let feedback_text = (try { $feedback_html | ^pup 'pre' text{} | str trim } catch { $feedback_html })
            print $feedback_text
        }
        break
    }
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
